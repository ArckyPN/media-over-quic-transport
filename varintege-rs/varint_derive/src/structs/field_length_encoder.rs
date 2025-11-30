use quote::{ToTokens, quote};
use syn::Ident;

use crate::utils::attributes::StructFieldAttributes;

pub struct FieldLengthEncoder<'a> {
    pub name: &'a Ident,
    pub attr: &'a StructFieldAttributes,
    pub length_required: bool,
}

impl<'a> ToTokens for FieldLengthEncoder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.attr.length {
            Some(ref ty) => {
                let field = &self.name;
                quote! {
                    let field_len = self.#field.len_bits()?;
                    let field_length = <#ty>::try_from(field_len / 8)?;
                    bits += field_length.encode(writer, None)?;
                    let field_len = Some(field_len);
                }
            }
            None if self.length_required => quote! {
                let field_len = Some(length - bits);
            },
            None => quote! {
                let field_len = None;
            },
        }
        .to_tokens(tokens);
    }
}
