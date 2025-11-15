use proc_macro_error2::abort;
use quote::quote;
use syn::{Ident, Lit, LitInt, Type, ext::IdentExt, parse::Parse, token::Bracket};

use crate::crate_name;

pub struct XMacro {
    pub ty: Type,
}

const X_IDENT_NUMBER: &str = "i";
const X_IDENT_TUPLE: &str = "tuple";
const X_IDENT_BINARY: &str = "b";

impl Parse for XMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let varint = crate_name();

        let ty = if input.peek2(Token![::]) || input.peek2(Token![<]) {
            // parse types
            input.parse()?
        } else if input.peek(Ident::peek_any) {
            // check for ident first
            let ident = input.parse::<Ident>()?;

            // three predefined idents correspond to specific types
            if ident == X_IDENT_NUMBER {
                syn::parse(quote! { #varint::core::types::Number }.into())?
                // TODO moq feature
            } else if ident == X_IDENT_TUPLE {
                syn::parse(quote! { #varint::core::types::Tuple }.into())?
                // TODO moq feature
            } else if ident == X_IDENT_BINARY {
                syn::parse(quote! { #varint::core::types::BinaryData }.into())?
            } else {
                // any others would be other types or paths
                syn::parse(quote! { #ident }.into())?
            }
        } else if input.peek(Lit) {
            // literals are always numbers
            let bits = input.parse::<Lit>()?;

            if input.peek(Token![..]) {
                // BitRange without start value
                input.parse::<Token![..]>()?;

                if input.peek(Lit) {
                    // BitRange with start and end
                    let end = input.parse::<Lit>()?;

                    syn::parse(quote! { #varint::core::types::BitRange<#bits, #end> }.into())?
                } else {
                    // BitRange with open end
                    syn::parse(quote! { #varint::core::types::BitRange<#bits> }.into())?
                }
            } else if input.peek(Token![=]) {
                // BitNumber with specific values
                input.parse::<Token![=]>()?;

                let start = if input.peek(Lit) {
                    // start value
                    input.parse::<Lit>()?
                } else if input.peek2(Token![..]) {
                    // use 0 as start value if none specified and is part of a range
                    Lit::Int(LitInt::new("0", input.span()))
                } else {
                    abort!(input.span(), "Expected value or ..")
                };

                if input.peek(Token![..]) {
                    // BitNumber with a number range
                    input.parse::<Token![..]>()?;

                    if input.peek(Lit) {
                        // BitNumber with a specified range limit
                        let end = input.parse::<Lit>()?;
                        syn::parse(
                            quote! { #varint::core::types::BitNumber<#bits, #start, #end> }.into(),
                        )?
                    } else {
                        // BitNumber with an open range
                        syn::parse(
                            quote! { #varint::core::types::BitNumber<#bits, #start> }.into(),
                        )?
                    }
                } else {
                    // BitNumber with a constant value
                    syn::parse(
                        quote! { #varint::core::types::BitNumber<#bits, #start, #start> }.into(),
                    )?
                }
            } else {
                // BitNumber unspecified value
                syn::parse(quote! { #varint::core::types::BitNumber<#bits> }.into())?
            }
        } else if input.peek(Token![..]) {
            // BitRange without starting limit
            input.parse::<Token![..]>()?;

            if input.peek(Lit) {
                // BitRange with specified range limit
                let end = input.parse::<Lit>()?;
                syn::parse(quote! { #varint::core::types::BitRange<0, #end> }.into())?
            } else {
                // BitRange without range (essentially the same as BinaryData)
                syn::parse(quote! { #varint::core::types::BitRange }.into())?
            }
        } else if input.peek(Bracket) {
            // anything enclosed in Brackets is turned into an Option
            let content;
            bracketed!(content in input);
            let ty = content.parse::<XMacro>()?;
            let ty = ty.ty;
            syn::parse(quote! { Option<#ty> }.into())?
        } else {
            abort!(input.span(), "Unknown Token")
        };

        // make it a Vec if next tokens are ; ...
        let ty = if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            input.parse::<Token![...]>()?;

            syn::parse(quote! { Vec<#ty> }.into())?
        } else {
            ty
        };

        Ok(Self { ty })
    }
}
