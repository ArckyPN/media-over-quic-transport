mod draft_ref;
mod enums;
mod macro_helper;
mod structs;
mod utils;
mod varint_enum;
mod x;

// TODO I should restrict all more stuff behind a moq feature

use draft_ref::DraftRefArgs;
use enums::ImplEnum;
use structs::ImplStruct;
use varint_enum::VarIntEnum;
use x::XMacro;

#[macro_use]
extern crate syn;

use proc_macro_error2::{abort, abort_call_site, proc_macro_error};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Data, DataStruct, DeriveInput, Expr, Fields, Ident, Type, spanned::Spanned};

const ATTRIBUTE: &str = "varint";
const MACRO_IDENT: &str = "x";
const VARINT_CRATE: &str = "varintege-rs";

// TODO check example: /home/philip/Code/playground/structopt/
// TODO for errors: https://docs.rs/proc-macro-error2/2.0.1/proc_macro_error2/index.html#guide

/// TODO docs
#[proc_macro_derive(VarInt, attributes(varint))]
#[proc_macro_error]
pub fn derive_var_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name_ident = &input.ident;

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => ImplStruct::new(name_ident, &input.attrs, fields).to_token_stream(),
        Data::Enum(e) => ImplEnum::new(name_ident, e, &input.attrs).to_token_stream(),
        _ => abort_call_site!("VarInt only supports non-tuple structs and enums"),
    }
    .into()
}

/// TODO docs
#[proc_macro]
#[proc_macro_error]
pub fn x(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let x_mac = parse_macro_input!(input as XMacro);
    let ty = x_mac.ty;
    quote! { #ty }.into()
}

/// Create an VarInt enum.
///
/// # Example
///
/// ```ignore
/// varint_enum! {
///     // optional length key-value pairs allow setting all struct or tuple fields
///     // with a specific length attribute
///     length {
///         // give the first (index 0) field of every tuple the length attribute x(i)
///         tuple[0] = x(i),
///         // the second field of every struct gets x(64) length
///         struct[1] = x(64),
///     }
///     /// all attributes are forwarded to the actual type
///     #[derive(Debug)]
///     #[varint(value = x(16))] // optional override of default value type `x(i)`
///     pub enum Message {
///         /// variant attribute are also forwarded
///         Empty = 0x00 // assign values for VarInt decoding
///         Anonym(
///             #[varint(length = x(8))] // explicitly give a length to a field
///             x!(..),
///             #[doc = "tuple field attributes are forwarded"]
///             x!(8)
///         ) = 0x01, // tuple variants supported
///         Full {
///             #[varint(length = x(i))] // struct field attributes are forwarded
///             name: x!(..),
///             data: SomeData,
///         } = 0x02,
///         Other(x!(..)) = 0x03,
///     }
/// }
///
///     // this will generate the following enum
///
///     /// all attributes are forwarded to the actual type
///     #[derive(Debug, VarInt)]
///     #[varint(value = x(16))]
///     pub enum Message {
///         /// variant attribute are also forwarded
///         #[varint(value = 0x00)]
///         Empty,
///         #[varint(value = 0x01)]
///         Anonym(
///             #[varint(length = x(8))] // when a field already had a specified length it overrides the global length
///             varintege_rs::core::types::BitRange,
///             /// tuple field attributes are forwarded
///             varintege_rs::core::types::BitNumber<8>,
///         ),
///         #[varint(value = 0x02)]
///         Full {
///             #[varint(length = x(i))]
///             name: varint::core::types::BitRange,
///             #[varint(length = x(64))] // global length is applied
///             data: SomeData,
///         },
///         #[varint(value = 0x03)]
///         Other(
///             #[varint(length = x(i))] // global length is applied
///             name: varint::core::types::BitRange,
///         ),
///     }
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn varint_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let vie = parse_macro_input!(input as VarIntEnum);
    quote! { #vie }.into()
}

// TODO doc + moq feature
#[proc_macro_attribute]
#[proc_macro_error]
pub fn draft_ref(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as DraftRefArgs);
    let mut input = parse_macro_input!(item as DeriveInput);

    let doc = args.to_doc_string(&input.ident);
    input.attrs.push(syn::parse_quote!( #[doc = #doc] ));
    quote! {
        // #[doc = #doc]
        #input
    }
    .into()
}

fn crate_name() -> Ident {
    match proc_macro_crate::crate_name(VARINT_CRATE) {
        Ok(proc_macro_crate::FoundCrate::Itself) => {
            Ident::new(&VARINT_CRATE.replace("-", "_"), Span::call_site())
        }
        Ok(proc_macro_crate::FoundCrate::Name(s)) => Ident::new(&s, Span::call_site()),
        Err(err) => abort_call_site!("{} is not a dependency, err: {}", VARINT_CRATE, err),
    }
}

fn parse_varint_type(expr: &Expr) -> Type {
    let s = expr.to_token_stream().to_string();
    if !s.starts_with(MACRO_IDENT) {
        abort!(expr.span(), "Invalid Expr, expected varint x-macro")
    }

    match expr {
        Expr::Call(call) => {
            let args = &call.args;
            // TODO verify args are valid

            let krayt = crate_name();
            match syn::parse(quote! { #krayt::x!(#args) }.into()) {
                Ok(v) => v,
                Err(err) => abort_call_site!("Invalid Macro: {}", err),
            }
        }
        _ => abort!(expr.span(), "expected a x() call, akin to the x! macro"),
    }
}
