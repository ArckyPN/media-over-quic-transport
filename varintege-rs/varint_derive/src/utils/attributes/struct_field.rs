use proc_macro_error2::abort;
use quote::ToTokens;
use syn::{Attribute, Expr, Type, parse::Parse, spanned::Spanned};

use super::When;
use crate::ATTRIBUTE;

const LENGTH_ATTR: &str = "length";
const WHEN_ATTR: &str = "when";
const COUNT_ATTR: &str = "count";

#[derive(Default, Clone)]
pub struct StructFieldAttributes {
    /// length in bits of the field
    pub length: Option<Type>,

    /// when the field should be Some(_)
    pub when: Option<When>,

    /// type of the number denoting the count of Vec<_>,
    /// must be a type that doesn't require a length itself
    /// TODO validate that it is a type which doesn't require a length!
    pub count: Option<Type>,
}

impl Parse for StructFieldAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = input.parse_terminated(Expr::parse, Token![,])?;

        let mut this = Self::default();
        for expr in exprs {
            match expr {
                Expr::Assign(assign) => match assign.left.to_token_stream().to_string().as_str() {
                    LENGTH_ATTR => this.length = Some(crate::parse_varint_type(&assign.right)),
                    COUNT_ATTR => this.count = Some(crate::parse_varint_type(&assign.right)),
                    _ => abort!(
                        assign.span(),
                        "unknown left side of assignment, expected {}",
                        LENGTH_ATTR
                    ),
                },
                Expr::Call(call) => match call.func.to_token_stream().to_string().as_str() {
                    WHEN_ATTR => {
                        let len = call.args.len();
                        if len != 1 {
                            abort!(call.span(), "exactly one argument expected, found: {}, len")
                        }

                        this.when = Some(When::new(call.args.first(), call.span()));
                    }
                    _ => abort!(call.span(), "unknown call, expected {}", WHEN_ATTR),
                },
                _ => abort!(expr.span(), "only assigns and calls are supported"),
            }
        }

        Ok(this)
    }
}

impl StructFieldAttributes {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut this = None;
        for attr in attrs {
            if !attr.path().is_ident(ATTRIBUTE) {
                continue;
            }
            if this.is_some() {
                abort!(
                    attr.span(),
                    "Duplicate Attribute: please use only one {}-Attribute",
                    ATTRIBUTE
                )
            }
            this = Some(match attr.parse_args() {
                Ok(v) => v,
                Err(err) => {
                    abort!(attr.path().span(), "Attribute Error: {}", err)
                }
            });
        }
        if let Some(this) = this {
            return this;
        }
        Self::default()
    }
}
