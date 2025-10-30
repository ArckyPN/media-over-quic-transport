use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{ spanned::Spanned, token::Brace, Expr, FieldsNamed, Ident, PathArguments, Type, TypePath};

use crate::{crate_name, utils::attributes::StructFieldAttributes};

pub struct ImplStruct {
    /// name of the struct being derived upon
    name: Ident,

    /// name of the crate
    varint: Ident,

    /// names of the fields
    field_names: Vec<Ident>,

    /// types of the fields
    field_tys: Vec<Type>,

    /// attributes of each field
    field_attrs: Vec<StructFieldAttributes>,
}

impl ImplStruct {
    pub fn new(name: &Ident, fields: &FieldsNamed) -> Self {
        let mut field_names = Vec::new();
        let mut field_tys = Vec::new();
        let mut field_attrs = Vec::new();
        for field in &fields.named {
            field_tys.push(field.ty.clone());

            match &field.ident {
                Some(name) => field_names.push(name.clone()),
                None => abort!(field.span(), "Unnamed Fields not supported")
            }

            let attr = StructFieldAttributes::from_attrs(&field.attrs);
            attr.validate_when(&field_names);
            if option_type(&field.ty).is_some() && attr.when.is_none() {
                abort!(field.span(), "Option values must have a when attribute!")
            }
            if vec_type(&field.ty).is_some() && attr.count.is_none() {
                abort!(field.span(), "Vec values must have a count attribute")
            }
            field_attrs.push(attr);
        }

        Self { name: name.clone(), varint: crate_name(), field_names, field_tys, field_attrs }
    }

    fn field_length_decoders(&self) -> Vec<TokenStream> {
        let varint = &self.varint;

        self.field_attrs.iter().map(|attr| {
            if let Some(f) = &attr.length {
                quote! {
                    let (field_len, len) = <#f as #varint::core::VarInt>::decode(reader, None)?;
                    bits += len;
                    let field_len = Some(field_len.number::<usize>() * 8);
                }
            } else if self.length_required() {
                quote! {
                    let field_len = Some(length - bits);
                }
            } else {
                quote! {
                    let field_len = None;
                }
            }
        }).collect()
    }

    fn field_length_encoders(&self) -> Vec<TokenStream> {
        self.field_attrs.iter().zip(&self.field_names).map(|(attr, field)| {
            match &attr.length {
                Some(ty) => quote! {
                    let field_len = self.#field.len_bits();
                    let field_length = <#ty>::try_from(field_len / 8)?;
                    bits += field_length.encode(writer, None)?;
                    let field_len = Some(field_len);
                },
                None if self.length_required() => quote! {
                    let field_len = Some(length - bits);
                },
                None => quote! {
                    let field_len = None;
                },
            }
        }).collect()
    }

    fn field_decoders(&self) -> Vec<TokenStream> {
        let varint = &self.varint;

        let field_length = self.field_length_decoders();

        self.field_names.iter().zip(field_length).zip(&self.field_tys).zip(&self.field_attrs).map(|(((field, field_length), field_ty), field_attr)| {
            if let Some(ty) = option_type(field_ty)
                    && let Some(when) = &field_attr.when {
                let pf = &when.field;
                let vals = &when.values;
                quote! {
                    let #field = if #(#pf == #vals)||* {
                        #field_length
                        let (field, len) = <#ty as #varint::core::VarInt>::decode(reader, field_len)?;
                        bits += len;
                        Some(field)
                    } else {
                        None
                    };
                }
            } else if let Some(ty) = vec_type(field_ty)
                    && let Some(count) = &field_attr.count {
                quote! {
                    let (count, len) = <#count as #varint::core::VarInt>::decode(reader, None)?;
                    bits += len;

                    let mut #field = Vec::new();
                    for _ in 0..count.number::<usize>() {
                        #field_length
                        let (field, len) = <#ty as #varint::core::VarInt>::decode(reader, field_len)?;
                        bits += len;
                        #field.push(field);
                    }
                }
            } else { 
                quote! {
                    #field_length
                    let (#field, len) = <#field_ty as #varint::core::VarInt>::decode(reader, field_len)?; 
                    bits += len;
                }
            }
        }).collect()
    }

    fn field_encoders(&self) -> Vec<TokenStream> {
        let field_length = self.field_length_encoders();
        
        self.field_names.iter().zip(&self.field_tys).zip(&self.field_attrs).zip(field_length).map(|(((field, field_ty), field_attr), field_length)| {
            if let Some(_ty) = option_type(field_ty) {
                quote! {
                    if let Some(val) = &self.#field {
                        #field_length
                        bits += val.encode(writer, field_len)?;
                    }
                }
            } else if let Some(_ty) = vec_type(field_ty)
                    && let Some(count) = &field_attr.count {
                quote! {
                    bits += <#count>::try_from(self.#field.len())?.encode(writer, None)?;
                    for element in &self.#field {
                        #field_length
                        bits += element.encode(writer, field_len)?;
                    }
                }
            } else {
                quote! {
                    #field_length
                    bits += self.#field.encode(writer, field_len)?; 
                }
            }
        }).collect()
    }

    fn len_bits(&self) -> Vec<TokenStream> {
        self.field_names.iter().zip(&self.field_tys).zip(&self.field_attrs).map(|((field, field_ty), field_attr)| {
            if let Some(_ty) = option_type(field_ty) {
                let length_handle = match &field_attr.length {
                    Some(ty) => quote! {
                        bits += <#ty>::try_from(val.len_bits() / 8).expect("# TODO").len_bits();
                    },
                    None => quote! {}
                };
                quote! {
                    if let Some(val) = &self.#field {
                        #length_handle
                        bits += val.len_bits();
                    }
                }
            } else if let Some(_ty) = vec_type(field_ty)
                    && let Some(count) = &field_attr.count {
                quote! {
                    bits += <#count>::try_from(self.#field.len()).expect("# TODO: same as with enum").len_bits();
                    for element in &self.#field {
                        bits += element.len_bits();
                    }
                }
            } else {
                let length_handle = match &field_attr.length {
                    Some(ty) => quote! {
                        bits += <#ty>::try_from(self.#field.len_bits() / 8).expect("# TODO").len_bits();
                    },
                    None => quote! {}
                };
                quote! {
                    #length_handle
                    bits += self.#field.len_bits();
                }
            }
        }).collect()
    }

    fn length_required(&self) -> bool {
        let last = self.field_attrs.len() -1;
        let typ_is_bit_range = |typ: &Type| -> bool {
            let typ = typ.to_token_stream().to_string();
            (typ.contains("..") && !typ.contains("...")) || typ.contains("BitRange")
        };

        self.field_attrs.iter().zip(&self.field_tys).zip(&self.field_names).enumerate().fold(false, |_acc, (idx, ((attr, typ), field))| {
            let is_bit_range = typ_is_bit_range(typ);
            if attr.length.is_none() && is_bit_range && idx != last {
                abort!(field.span(), "x!(A..B) requires a length attribute, when not the last field")
            }

            attr.length.is_none() && is_bit_range && idx == last 
        })
    }

    fn impl_varint(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let varint = &self.varint;
        let field_names = &self.field_names;

        let field_decoders = self.field_decoders();
        let field_encoders = self.field_encoders();
        let len_bits = self.len_bits();
        let length_required = self.length_required();

        let length_arg = if length_required {
            format_ident!("length")
        } else {
            format_ident!("_length")
        };

        let length_unwrap = if length_required {
            quote! {
                let length = length.context(ctx::MissingLengthSnafu)?;
            }
        } else {
            quote! {}
        };

        let length_validate = if length_required {
            quote! {
                snafu::ensure!(bits == length, ctx::LengthMismatchSnafu { expected: length, got: bits });
            }
        } else {
            quote! {}
        };

        quote! {
            impl #varint::VarInt for #name
        }
        .to_tokens(tokens);
        Brace::default().surround(tokens, |tokens| {quote! {
            type Error = #varint::Error;

            fn decode<R>(reader: &mut R, #length_arg: Option<usize>) -> Result<(Self, usize), Self::Error>
            where
                R: #varint::Reader,
                Self: std::marker::Sized,
            {
                use #varint::{
                    error::ctx,
                    snafu::{self, ResultExt, OptionExt}
                };

                // count how many bits have been read
                let mut bits = 0;
                #length_unwrap

                #( #field_decoders )*

                #length_validate

                Ok((Self { #(#field_names),* }, bits))
            }

            fn encode<W>(&self, writer: &mut W, #length_arg: Option<usize>) -> Result<usize, Self::Error>
            where
                W: #varint::Writer,
            {
                use #varint::{
                    error::ctx,
                    snafu::{self, ResultExt, OptionExt}
                };

                let mut bits = 0;
                #length_unwrap

                #( #field_encoders )*

                #length_validate

                Ok(bits)
            }
            
            fn len_bits(&self) -> usize {
                let mut bits = 0;
                #( #len_bits )*
                bits
            }

            fn length_required() -> bool {
                #length_required
            }
        }.to_tokens(tokens);});
    }
}

impl ToTokens for ImplStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_varint(tokens);
    }
}

fn option_type(ty: &Type) -> Option<Box<dyn ToTokens>> {
    match ty {
        Type::Macro(m) => {
            if m.mac.path.is_ident("x") {
                let tok = &m.mac.tokens.to_string();
                if tok.starts_with("[") && tok.ends_with("]") {
                    let inner = m.mac.tokens.to_string();
                    let inner = inner.trim_start_matches('[').trim_end_matches(']');
                    // let inner = format_ident!("{}", inner);
                    let inner = match syn::parse_str::<Expr>(inner) {
                        Ok(v) => v,
                        Err(err) => abort!(m.mac.tokens.span(), "invalid token: {}", err),
                    };
                    Some(Box::new(quote! { x!(#inner) }))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Type::Path(p) => {
            get_generic(p, "Option")
        }
        _ => None,
    }
}

fn vec_type(ty: &Type) -> Option<Box<dyn ToTokens>> {
    match ty {
        Type::Macro(m) => {
            let parts = m.mac.tokens.to_string();
            let mut parts = parts.split(";");

            let ident = parts.next()?;

            let dots = parts.next()?;

            if !dots.ends_with("...") {
                return None;
            }

            let ident = Ident::new(ident.trim(), m.span());
            Some(Box::new(quote! {   x!(#ident)   }))
        }
        Type::Path(p) => {
            get_generic(p, "Vec")
        }
        _ => None,
    }
}

fn get_generic(p: &TypePath, ty: &str) -> Option<Box<dyn ToTokens>> {
    if p.qself.is_some() {
        return None;
    }
    let path_ident = p
        .path
        .segments
        .iter()
        .find(|s| s.to_token_stream().to_string().contains(ty))?;

    match &path_ident.arguments {
        PathArguments::AngleBracketed(typ) => {
            let typ = typ.args.first()?;
            Some(Box::new(quote! { #typ }))
        }
        x => abort!(x.span(), "Malformed {}<_>", ty),
    }
}
