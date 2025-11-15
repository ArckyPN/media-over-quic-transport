use proc_macro_error2::abort;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Attribute, Expr, Lit, spanned::Spanned};

use crate::ATTRIBUTE;

const VALUE_ATTR: &str = "value";

pub struct EnumVariantAttributes {
    pub value: Lit,
}

impl syn::parse::Parse for EnumVariantAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: Expr = input.parse()?;

        let expr = match expr {
            Expr::Assign(assign) => assign,
            _ => abort!(expr.span(), "only assigns are support"),
        };

        if expr.left.to_token_stream().to_string() != VALUE_ATTR {
            abort!(expr.left.span(), "unknown ident, expected value")
        }

        let value = match syn::parse(expr.right.to_token_stream().into()) {
            Ok(v) => v,
            Err(err) => abort!(expr.right.span(), "Invalid Value: {}", err),
        };

        Ok(Self { value })
    }
}

impl EnumVariantAttributes {
    pub fn from_attrs(attrs: &[Attribute], ctx: Span) -> Self {
        let mut a = None;
        for attr in attrs {
            if !attr.path().is_ident(ATTRIBUTE) {
                continue;
            }
            a = match attr.parse_args() {
                Ok(v) => Some(v),
                Err(err) => abort!(attr.path().span(), "Attribute Error: {}", err),
            }
        }
        let Some(a) = a else {
            abort!(ctx, "Missing value")
        };
        a
    }
}
