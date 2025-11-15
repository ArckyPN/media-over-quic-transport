use proc_macro_error2::{abort, emit_error};
use quote::ToTokens;
use syn::Lit;

pub mod attributes;
pub mod fields;

pub fn add_value_unique(val: Lit, values: &mut Vec<Lit>) {
    if let Some(dup) = values
        .iter()
        .find(|v| v.to_token_stream().to_string() == val.to_token_stream().to_string())
    {
        emit_error!(dup.span(), "first occurrence of duplicate");
        abort!(val.span(), "duplicate value")
    }
    values.push(val);
}
