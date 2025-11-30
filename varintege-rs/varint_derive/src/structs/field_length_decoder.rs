use quote::{ToTokens, quote};
use syn::Ident;

use crate::utils::attributes::StructFieldAttributes;

pub struct FieldLengthDecoder<'a> {
    pub varint: &'a Ident,
    pub attr: &'a StructFieldAttributes,
    pub length_required: bool,
}

impl<'a> ToTokens for FieldLengthDecoder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(f) = &self.attr.length {
            let varint = self.varint;
            quote! {
                let (field_len, len) = <#f as #varint::core::VarInt>::decode(reader, None)?;
                bits += len;
                let field_len = Some(field_len.number::<usize>() * 8);
            }
        } else if self.length_required {
            quote! {
                let field_len = Some(length - bits);
            }
        } else {
            quote! {
                let field_len = None;
            }
        }
        .to_tokens(tokens);
    }
}
