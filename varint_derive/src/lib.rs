use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(VarInt)]
pub fn derive_var_int(_input: TokenStream) -> TokenStream {
    // TODO proc_macro, at the end when the individual parts are working
    quote! {}.into()
}
