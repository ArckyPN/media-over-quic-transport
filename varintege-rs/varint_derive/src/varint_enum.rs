use std::collections::HashMap;

use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    Attribute, Expr, Ident, Lit, Type, Visibility,
    ext::IdentExt,
    parse::Parse,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Paren},
};

use crate::{ATTRIBUTE, crate_name};

const LENGTH: &str = "length";
const TUPLE: &str = "tuple";
const STRUCT: &str = "struct";

pub struct VarIntEnum {
    meta: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    variants: Punctuated<Variant, Token![,]>,
}

impl ToTokens for VarIntEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let varint = crate_name();
        quote! {#[derive(#varint::VarInt)]}.to_tokens(tokens);
        tokens.append_all(&self.meta);
        self.vis.to_tokens(tokens);
        quote! { enum }.to_tokens(tokens);
        self.name.to_tokens(tokens);
        Brace::default().surround(tokens, |f| self.variants.to_tokens(f));
    }
}

impl Parse for VarIntEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lengths = if input.peek(Ident::peek_any) {
            let i2 = input.fork();
            let peek = i2.parse::<Ident>()?;
            if peek == LENGTH {
                input.parse::<Ident>()?;
                let content;
                braced!(content in input);
                Some(content.parse::<Lengths>()?)
            } else {
                None
            }
        } else {
            None
        };
        let meta = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        input.parse::<Token![enum]>()?;
        let name = input.parse()?;

        let content;
        braced!(content in input);
        let mut variants = content.parse_terminated(Variant::parse, Token![,])?;

        if let Some(lengths) = lengths {
            for (idx, token) in &lengths.tuples {
                for variant in variants.iter_mut() {
                    if let Some(VariantFields::Tuple(tup)) = &mut variant.fields
                        && let Some(tup) = tup.get_mut(*idx)
                    {
                        if has_length_attr(&tup.meta) {
                            continue;
                        }
                        tup.meta.push(parse_quote!( #[varint(length = #token)] ));
                    }
                }
            }

            for (idx, token) in &lengths.structs {
                for variant in variants.iter_mut() {
                    if let Some(VariantFields::Struct(s)) = &mut variant.fields
                        && let Some(s) = s.get_mut(*idx)
                    {
                        if has_length_attr(&s.meta) {
                            continue;
                        }
                        s.meta.push(parse_quote!( #[varint(length = #token)] ));
                    }
                }
            }
        }

        Ok(Self {
            meta,
            vis,
            name,
            variants,
        })
    }
}

struct Lengths {
    tuples: HashMap<usize, TokenStream>,
    structs: HashMap<usize, TokenStream>,
}

impl Parse for Lengths {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tuples = HashMap::new();
        let mut structs = HashMap::new();

        for (ident, idx, ty) in input.parse_terminated(
            |input| {
                let ident = if input.peek(Token![struct]) {
                    input.parse::<Token![struct]>()?;
                    STRUCT.to_owned()
                } else {
                    let ident = input.parse::<Ident>()?;
                    if ident != TUPLE {
                        abort!(
                            ident.span(),
                            "unexpected ident, expected {} or {}",
                            TUPLE,
                            STRUCT
                        )
                    }
                    TUPLE.to_owned()
                };

                let content;
                bracketed!(content in input);

                let idx = content.parse::<Lit>()?;
                let idx = match idx {
                    Lit::Int(lit) => lit.base10_parse::<usize>()?,
                    _ => abort!(idx.span(), "expected number"),
                };

                input.parse::<Token![=]>()?;

                let x = input.parse::<Expr>()?;
                match &x {
                    Expr::Call(call) => {
                        if call.func.to_token_stream().to_string() != "x" {
                            abort!(call.func.span(), "expected x macro")
                        }
                    }
                    _ => abort!(x.span(), "expected x macro"),
                }

                Ok((ident, idx, x.to_token_stream()))
            },
            Token![,],
        )? {
            match ident.as_str() {
                TUPLE => tuples.insert(idx, ty),
                STRUCT => structs.insert(idx, ty),
                _ => unreachable!(""),
            };
        }

        Ok(Self { tuples, structs })
    }
}

struct Variant {
    meta: Vec<Attribute>,
    name: Ident,
    value: Lit,
    fields: Option<VariantFields>,
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = &self.value;
        let s = format!("# Key: `{}`", quote! { #value });
        let attr: Attribute = parse_quote!( #[doc = #s] );
        attr.to_tokens(tokens);
        tokens.append_all(&self.meta);
        let attr: Attribute = parse_quote!( #[varint(value = #value)] );
        attr.to_tokens(tokens);
        self.name.to_tokens(tokens);
        if let Some(fields) = &self.fields {
            quote! { #fields }.to_tokens(tokens);
        }
    }
}

impl Parse for Variant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;

        let content;
        let fields = if input.peek(Paren) {
            parenthesized!(content in input);
            Some(VariantFields::Tuple(
                content.parse_terminated(Tuple::parse, Token![,])?,
            ))
        } else if input.peek(Brace) {
            braced!(content in input);
            Some(VariantFields::Struct(
                content.parse_terminated(Struct::parse, Token![,])?,
            ))
        } else {
            None
        };

        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        match value {
            Lit::Int(_) => {}
            _ => abort!(value.span(), "expected number"),
        }

        Ok(Self {
            meta,
            name,
            value,
            fields,
        })
    }
}

enum VariantFields {
    Struct(Punctuated<Struct, Token![,]>),
    Tuple(Punctuated<Tuple, Token![,]>),
}

impl ToTokens for VariantFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(s) => Brace::default().surround(tokens, |f| s.to_tokens(f)),
            Self::Tuple(t) => Paren::default().surround(tokens, |f| t.to_tokens(f)),
        }
    }
}

struct Struct {
    meta: Vec<Attribute>,
    name: Ident,
    ty: Type,
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.meta);
        self.name.to_tokens(tokens);
        quote! { : }.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;

        Ok(Self { meta, name, ty })
    }
}

struct Tuple {
    meta: Vec<Attribute>,
    ty: Type,
}

impl ToTokens for Tuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.meta);
        self.ty.to_tokens(tokens);
    }
}
impl Parse for Tuple {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta = input.call(Attribute::parse_outer)?;
        let ty = input.parse()?;

        Ok(Self { meta, ty })
    }
}

fn has_length_attr(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident(ATTRIBUTE) {
            continue;
        }
        if attr.to_token_stream().to_string().contains(LENGTH) {
            return true;
        }
    }
    false
}
