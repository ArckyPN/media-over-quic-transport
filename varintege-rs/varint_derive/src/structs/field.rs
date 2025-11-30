use proc_macro_error2::abort;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, Type, spanned::Spanned};

use crate::utils::attributes::When;

use {
    super::{FieldLengthDecoder, FieldLengthEncoder, option_type, vec_type},
    crate::utils::attributes::StructFieldAttributes,
};

pub struct Field {
    pub name: Ident,
    pub ty: Type,
    pub attr: StructFieldAttributes,
    pub span: Span,
}

impl Field {
    pub fn validate(&self, previous_fields: &[Self]) {
        if let Some(when) = &self.attr.when
            && !previous_fields.iter().any(|pf| when.field == pf.name)
        {
            abort!(
                when.field.span(),
                "Field {} does not exist as a previous field",
                when.field
            )
        }

        if option_type(&self.ty).is_some() && self.attr.when.is_none() {
            abort!(self.ty.span(), "Option values must have a when attribute!")
        }
        if vec_type(&self.ty).is_some() && self.attr.count.is_none() {
            abort!(self.ty.span(), "Vec values must have a count attribute")
        }
    }

    pub fn encoder(&self, length_required: bool) -> TokenStream {
        let name = &self.name;

        let field_length = FieldLengthEncoder {
            name,
            attr: &self.attr,
            length_required,
        }
        .into_token_stream();

        if let Some(_ty) = option_type(&self.ty) {
            quote! {
                if let Some(val) = &self.#name {
                    #field_length
                    bits += val.encode(writer, field_len)?;
                }
            }
        } else if let Some(_ty) = vec_type(&self.ty)
            && let Some(count) = &self.attr.count
        {
            quote! {
                bits += <#count>::try_from(self.#name.len())?.encode(writer, None)?;
                for element in &self.#name {
                    #field_length
                    bits += element.encode(writer, field_len)?;
                }
            }
        } else {
            quote! {
                #field_length
                bits += self.#name.encode(writer, field_len)?;
            }
        }
    }

    pub fn decoder(&self, varint: &Ident, length_required: bool) -> TokenStream {
        let field_length = FieldLengthDecoder {
            varint,
            attr: &self.attr,
            length_required,
        }
        .into_token_stream();

        let name = &self.name;
        let ty = &self.ty;

        if let Some(ty) = option_type(&self.ty)
            && let Some(when) = &self.attr.when
        {
            let When { field, values } = when;

            quote! {
                let #name = if #(#field == #values)||* {
                    #field_length
                    let (field, len) = <#ty as #varint::core::VarInt>::decode(reader, field_len)?;
                    bits += len;
                    Some(field)
                } else {
                    None
                };
            }
        } else if let Some(ty) = vec_type(&self.ty)
            && let Some(count) = &self.attr.count
        {
            quote! {
                let (count, len) = <#count as #varint::core::VarInt>::decode(reader, None)?;
                bits += len;

                let mut #name = Vec::new();
                for _ in 0..count.number::<usize>() {
                    #field_length
                    let (field, len) = <#ty as #varint::core::VarInt>::decode(reader, field_len)?;
                    bits += len;
                    #name.push(field);
                }
            }
        } else {
            quote! {
                #field_length
                let (#name, len) = <#ty as #varint::core::VarInt>::decode(reader, field_len)?;
                bits += len;
            }
        }
    }

    pub fn len_bits(&self) -> TokenStream {
        let name = &self.name;

        if let Some(_ty) = option_type(&self.ty) {
            let length = self.attr.length.as_ref().map(|ty| {
                quote! {
                    bits += <#ty>::try_from(val.len_bits()? / 8)?.len_bits()?;
                }
            });
            quote! {
                if let Some(val) = &self.#name {
                    #length
                    bits += val.len_bits()?;
                }
            }
        } else if let Some(_ty) = vec_type(&self.ty)
            && let Some(count) = &self.attr.count
        {
            quote! {
                bits += <#count>::try_from(self.#name.len())?.len_bits()?;
                for element in &self.#name {
                    bits += element.len_bits()?;
                }
            }
        } else {
            let length = self.attr.length.as_ref().map(|ty| {
                quote! {
                    bits += <#ty>::try_from(self.#name.len_bits()? / 8)?.len_bits()?;
                }
            });
            quote! {
                #length
                bits += self.#name.len_bits()?;
            }
        }
    }
}
