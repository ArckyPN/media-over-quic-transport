use proc_macro_error2::abort;
use quote::ToTokens;
use syn::{Attribute, Expr, Type, parse::Parse, spanned::Spanned};

use crate::ATTRIBUTE;

const LEN_ATTR: &str = "length";

#[derive(Default)]
pub struct FieldAttributes {
    pub length: Option<Type>,
}

impl Parse for FieldAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = input.parse_terminated(Expr::parse, Token![,])?;

        let mut this = Self::default();
        for expr in exprs {
            match expr {
                Expr::Assign(assign) => match assign.left.to_token_stream().to_string().as_str() {
                    LEN_ATTR => this.length = Some(crate::parse_varint_type(&assign.right)),
                    _ => abort!(assign.span(), "unknown ident, expected {}", LEN_ATTR),
                },
                _ => abort!(expr.span(), "only assigns are supported"),
            }
        }
        Ok(this)
    }
}

impl FieldAttributes {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut a = Self::default();
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
