use std::collections::HashMap;

use convert_case::{Case, Casing};
use proc_macro_error2::{abort, abort_call_site};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Expr, FieldsNamed, Ident, LitStr, Meta, Path, parse::Parse, spanned::Spanned,
};

use crate::{ATTRIBUTE, crate_name};

const PARAM_FIELD: &str = "parameters";

// TODO ideally parse this from the actual Parameter enums
/// # TODO doc for all params
pub mod setup {
    pub mod client {
        crate::macro_helper::parameter_map! {
            // TODO correct types
            /// Only used when using QUIC as transport
            /// protocol instead of WebTransport
            path = 0x01 => Path as String,
            max_request_id = 0x02 => MaxRequestId as u64,
            auth_token = 0x03 => AuthorizationToken as crate::types::Token,
            authority = 0x05 => Authority as String,
            moqt_implementation = 0x07 => MoqtImplemenation as String, // draft spec also has this as 0x05 (same as authority)
        }
    }

    pub mod server {
        crate::macro_helper::parameter_map! {
            // TODO correct types
            /// # TODO doc
            max_request_id = 0x02 => MaxRequestId as u64,
            auth_token = 0x03 => AuthorizationToken as crate::types::Token,
            max_auth_token_cache_size = 0x04 => MaxAuthorizationTokenCacheSize as u64,
            moqt_implementation = 0x07 => MoqtImplemenation as String, // draft spec also has this as 0x05 (same as authority)
        }
    }
}
pub mod general {
    crate::macro_helper::parameter_map! {
        delivery_timeout = 0x02 => DeliveryTimeout as std::time::Duration,
        /// # TODO doc
        auth_token = 0x03 => AuthorizationToken as crate::types::Token,
        max_cache_duration = 0x04 => MaxCacheDuration as std::time::Duration,
    }
}

#[derive(Default)]
pub struct StructAttrs {
    pub parameters: Vec<Ident>,
    // TODO add and_then: Option<Path> (a fn to call after decode to validate the result)
}

impl Parse for StructAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = input.parse_terminated(Expr::parse, Token![,])?;

        let mut this = Self::default();
        for expr in exprs {
            match expr {
                Expr::Call(params) => match params.func.to_token_stream().to_string().as_str() {
                    PARAM_FIELD => {
                        this.parameters = params
                            .args
                            .iter()
                            .map(|arg| {
                                syn::parse::<Ident>(arg.to_token_stream().into()).unwrap_or_else(
                                    |err| abort!(arg.span(), "Invalid Ident: {}", err),
                                )
                            })
                            .collect::<Vec<_>>()
                    }
                    x => abort!(
                        params.span(),
                        "unknown ident {x:?}, expected {}",
                        PARAM_FIELD
                    ),
                },
                _ => abort!(expr.span(), "Unknown attribute, expected {}", PARAM_FIELD,),
            }
        }

        Ok(this)
    }
}

impl StructAttrs {
    pub fn new(attrs: &[Attribute], fields: &FieldsNamed) -> Self {
        let attr = Self::from_attributes(attrs);

        if !attr.parameters.is_empty() && !Self::has_parameters_field(fields) {
            abort_call_site!("Missing parameters field")
        }

        attr
    }

    fn from_attributes(attrs: &[Attribute]) -> Self {
        let mut a = Self::default();
        for attr in attrs {
            if !attr.path().is_ident(ATTRIBUTE) {
                continue;
            }
            a = match attr.parse_args() {
                Ok(v) => v,
                Err(err) => abort!(attr.path().span(), "Attribute Error: {}", err),
            }
        }
        a
    }

    fn has_parameters_field(fields: &FieldsNamed) -> bool {
        for field in &fields.named {
            if let Some(ident) = &field.ident
                && ident == PARAM_FIELD
            {
                return true;
            }
        }
        false
    }

    fn quote_params(
        &self,
        name: &Ident,
        parameters: &[Ident],
        parameter_map: &'static HashMap<&'static str, (u32, Vec<String>, String, String)>,
        prefix: &str,
    ) -> TokenStream {
        let varint = crate_name();

        let parameter_ty = format_ident!("{prefix}Parameter");
        let fns = parameter_map
            .iter()
            .map(|(k, (v, docs, variant, ty))| {
                if parameters.contains(&format_ident!("{k}")) {
                    let ident = format_ident!("{}", k);
                    let variant = format_ident!("{variant}");
                    let unreachable_literal = LitStr::new(
                        &format!("key {v:#X} is always be of type {variant}"),
                        name.span(),
                    );
                    let docs = docs.iter().map(|d| syn::parse_str::<Meta>(d).unwrap());
                    let ty: Path = syn::parse_str(ty).expect("won't fail");
                    quote! {
                        #(
                            #[#docs]
                        )*
                        pub fn #ident(&self) -> Option<&#ty> {
                            let key: #varint::x!(i) = #v.into();
                            self.parameters.get(&key).map(|p| match p {
                                crate::types::#parameter_ty::#variant(t) => t,
                                _ => unreachable!(#unreachable_literal)
                            })
                        }
                    }
                } else {
                    quote! {}
                }
            })
            .collect::<Vec<_>>();

        let builder_mod = format_ident!("{}_builder", name.to_string().to_case(Case::Snake));
        let builder_struct = format_ident!("{name}Builder");
        let param_enum = format_ident!("{prefix}Parameter");

        let setters = parameter_map
            .iter()
            .map(|(k, (v, docs, variant, ty))| {
                let fn_name =
                    format_ident!("{k}" /* variant.to_case(convert_case::Case::Snake) */,);
                let docs = docs.iter().map(|d| syn::parse_str::<Meta>(d).unwrap());
                let ty: Path = syn::parse_str(ty).expect("won't fail");
                let variant = format_ident!("{variant}");

                quote! {
                    #(
                        #[#docs]
                    )*
                    fn #fn_name<V>(mut self, value: V) -> Self
                    where
                        V: Into<#ty>
                    {
                        self.parameters.insert(
                            <#varint::x!(i)>::from(#v as u32),
                            crate::types::parameter::#param_enum::#variant(value.into())
                        );
                        self
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl #name {
                #( #fns )*

                pub fn parameter<T>(&self, key: T) -> Option<&crate::types::#parameter_ty>
                where
                    T: Into<#varint::x!(i)>,
                {
                    let key: #varint::x!(i) = key.into();
                    self.parameters.get(&key)
                }

                pub fn parameter_mut<T>(&mut self, key: T) -> Option<&mut crate::types::#parameter_ty>
                where
                    T: Into<#varint::x!(i)>,
                {
                    let key: #varint::x!(i) = key.into();
                    self.parameters.get_mut(&key)
                }
            }

            impl<S: #builder_mod::State> #builder_struct<S> {
                #(
                    #setters
                )*

                /// Adds a generic number parameter.
                fn number<K, V>(mut self, key: K, value: V) -> Self
                where
                    K: Into<#varint::x!(i)>,
                    V: Into<#varint::x!(i)>,
                {
                    self.parameters
                        .insert(key.into(), crate::types::parameter::#param_enum::Number(value.into()));
                    self
                }

                /// Adds a generic bytes parameter.
                fn bytes<K, V>(mut self, key: K, value: V) -> Self
                where
                    K: Into<#varint::x!(i)>,
                    V: Into<#varint::x!(..)>,
                {
                    self.parameters
                        .insert(key.into(), crate::types::parameter::#param_enum::Bytes(value.into()));
                    self
                }
            }
        }
    }

    fn quote_client_setup(&self, name: &Ident) -> TokenStream {
        self.quote_params(
            name,
            &[
                // TODO define properly above
                format_ident!("path"),
                format_ident!("max_request_id"),
                format_ident!("auth_token"),
                format_ident!("authority"),
                format_ident!("moqt_implementation"),
            ],
            setup::client::parameter_map(),
            "ClientSetup",
        )
    }

    fn quote_server_setup(&self, name: &Ident) -> TokenStream {
        self.quote_params(
            name,
            &[
                // TODO define properly above
                format_ident!("max_request_id"),
                format_ident!("auth_token"),
                format_ident!("max_auth_token_cache_size"),
                format_ident!("moqt_implementation"),
            ],
            setup::server::parameter_map(),
            "ServerSetup",
        )
    }

    fn quote_general(&self, name: &Ident, parameters: &[Ident]) -> TokenStream {
        self.quote_params(name, parameters, general::parameter_map(), "")
    }

    pub fn quote(&self, name: &Ident) -> TokenStream {
        if name == "ClientSetup" {
            self.quote_client_setup(name)
        } else if name == "ServerSetup" {
            self.quote_server_setup(name)
        } else if !self.parameters.is_empty() {
            self.quote_general(name, &self.parameters)
        } else {
            quote! {}
        }
    }
}
