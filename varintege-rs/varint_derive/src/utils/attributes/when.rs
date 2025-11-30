use proc_macro_error2::abort;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Expr, Ident, Lit, LitInt, spanned::Spanned};

#[derive(Clone)]
pub struct When {
    pub field: Ident,
    pub values: Vec<Lit>,
}

impl When {
    pub fn new(expr: Option<&Expr>, ctx: Span) -> Self {
        match expr {
            Some(Expr::Assign(assign)) => {
                let field = Ident::new(&assign.left.to_token_stream().to_string(), assign.span());

                let values = assign
                    .right
                    .to_token_stream()
                    .to_string()
                    .split("||")
                    .map(|t| Lit::Int(LitInt::new(t.trim(), Span::call_site())))
                    .collect::<Vec<_>>();

                Self { field, values }
            }
            _ => abort!(ctx, "only assign is supported"),
        }
    }
}
