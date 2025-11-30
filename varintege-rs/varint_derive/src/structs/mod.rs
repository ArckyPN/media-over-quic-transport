mod attrs;
mod field;
mod field_length_decoder;
mod field_length_encoder;

use {
    crate::{crate_name, utils::attributes::StructFieldAttributes},
    attrs::StructAttrs,
    field::Field,
    field_length_decoder::FieldLengthDecoder,
    field_length_encoder::FieldLengthEncoder,
    proc_macro_error2::abort,
    proc_macro2::TokenStream,
    quote::{ToTokens, format_ident, quote},
    syn::{Attribute, Expr, FieldsNamed, Ident, PathArguments, Type, TypePath, spanned::Spanned},
};

pub struct ImplStruct {
    /// name of the struct being derived upon
    name: Ident,

    /// name of the crate
    varint: Ident,

    /// attributes of the struct
    attr: StructAttrs,

    /// struct fields
    fields: Vec<Field>,
}

impl ImplStruct {
    pub fn new(name: &Ident, attrs: &[Attribute], named_fields: &FieldsNamed) -> Self {
        let mut fields = Vec::new();

        let attr = StructAttrs::new(attrs, named_fields);

        for field in &named_fields.named {
            let field = Field {
                span: name.span(),
                name: field
                    .ident
                    .clone()
                    .unwrap_or_else(|| abort!(field.span(), "Unnamed Fields not supported")),
                ty: field.ty.clone(),
                attr: StructFieldAttributes::from_attrs(&field.attrs),
            };

            field.validate(&fields);

            fields.push(field);
        }

        Self {
            name: name.clone(),
            varint: crate_name(),
            attr,
            fields,
        }
    }

    fn length_required(&self) -> bool {
        let last = self.fields.len() - 1;
        let typ_is_bit_range = |typ: &Type| -> bool {
            let typ = typ.to_token_stream().to_string();
            (typ.contains("..") && !typ.contains("...")) || typ.contains("BitRange")
        };

        self.fields
            .iter()
            .enumerate()
            .fold(false, |_acc, (idx, field)| {
                let is_bit_range = typ_is_bit_range(&field.ty);
                if field.attr.length.is_none() && is_bit_range && idx != last {
                    abort!(
                        field.span,
                        "x!(A..B) requires a length attribute, when not the last field"
                    )
                }

                field.attr.length.is_none() && is_bit_range && idx == last
            })
    }

    fn impl_varint(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let varint = &self.varint;
        let field_names = &self.fields.iter().map(|f| &f.name).collect::<Vec<_>>();

        let field_decoders = self
            .fields
            .iter()
            .map(|f| f.decoder(varint, self.length_required()));

        let field_encoders = self
            .fields
            .iter()
            .map(|f| f.encoder(self.length_required()));
        let len_bits = self.fields.iter().map(|f| f.len_bits());

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

        self.attr.quote(name).to_tokens(tokens);

        quote! {
            impl #varint::VarInt for #name {
                type Error = #varint::Error;

                fn decode<R>(reader: &mut R, #length_arg: Option<usize>) -> Result<(Self, usize), Self::Error>
                where
                    R: #varint::Reader,
                    Self: std::marker::Sized,
                {
                    use #varint::{
                        error::ctx,
                        snafu::{self, ResultExt, OptionExt},
                        VarIntNumber,
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

                fn len_bits(&self) -> Result<usize, Self::Error> {
                    let mut bits = 0;
                    #( #len_bits )*
                    Ok(bits)
                }

                fn length_required() -> bool {
                    #length_required
                }
            }
        }
        .to_tokens(tokens);
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
        Type::Path(p) => get_generic(p, "Option"),
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
        Type::Path(p) => get_generic(p, "Vec"),
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
