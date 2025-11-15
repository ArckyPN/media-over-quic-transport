use proc_macro_error2::abort;
use quote::{ToTokens, quote};
use syn::{Attribute, Expr, Type, spanned::Spanned};

use crate::{ATTRIBUTE, crate_name};

const VALUE_ATTR: &str = "value";

pub struct EnumAttributes {
    pub value: Type,
}

impl syn::parse::Parse for EnumAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = input.parse_terminated(Expr::parse, Token![,])?;

        let mut this = Self::default();
        for expr in exprs {
            match expr {
                Expr::Assign(assign) => match assign.left.to_token_stream().to_string().as_str() {
                    VALUE_ATTR => {
                        this.value = crate::parse_varint_type(&assign.right);
                    }
                    _ => abort!(assign.span(), "unknown ident, expected {}", VALUE_ATTR),
                },
                _ => abort!(expr.span(), "only assigns are supported"),
            }
        }

        Ok(this)
    }
}

impl Default for EnumAttributes {
    fn default() -> Self {
        let varint = crate_name();
        Self {
            value: syn::parse(quote! { #varint::x!(i) }.into()).unwrap(),
        }
    }
}

impl EnumAttributes {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut a = EnumAttributes::default();
        for attr in attrs {
            if !attr.path().is_ident(ATTRIBUTE) {
                continue;
            }
            a = match attr.parse_args() {
                Ok(v) => v,
                Err(err) => {
                    abort!(attr.path().span(), "Attribute Error: {}", err)
                }
            }
        }
        a
    }
}
