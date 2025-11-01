use std::collections::HashMap;

use darling::FromAttributes;
use proc_macro_error2::abort_call_site;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, FieldsNamed, Ident, LitStr, Meta, Path};

const PARAM_FIELD: &str = "parameters";

pub(crate) trait Getter {
    fn get(&self, s: &str) -> bool;
}

// TODO ideally parse this from the actual Parameter enums
/// # TODO doc for all params
pub mod setup {
    pub mod client {
        crate::macro_helper::parameter_map! {
            // TODO correct types
            /// Only used when using QUIC as transport
            /// protocol instead of WebTransport
            path = 0x01 => Path as String,
            max_request_id = 0x02 => MaxRequestId as String,
            auth_token = 0x03 => AuthorizationToken as crate::types::Token,
            max_auth_token_cache_size = 0x04 => MaxAuthorizationTokenCacheSize as String,
            authority = 0x05 => Authority as String,
            moqt_implementation = 0x06 => MoqtImplemenation as String, // draft spec also has this as 0x05 (same as authority)
        }
    }

    pub mod server {
        crate::macro_helper::parameter_map! {
            // TODO correct types
            /// # TODO doc
            max_request_id = 0x02 => MaxRequestId as String,
            auth_token = 0x03 => AuthorizationToken as crate::types::Token,
            max_auth_token_cache_size = 0x04 => MaxAuthorizationTokenCacheSize as String,
            moqt_implementation = 0x06 => MoqtImplemenation as String, // draft spec also has this as 0x05 (same as authority)
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

#[derive(FromAttributes, Default)]
#[darling(attributes(varint))]
pub struct StructAttrs {
    parameters: Option<general::Parameters>,
}

impl StructAttrs {
    pub fn new(attrs: &[Attribute], fields: &FieldsNamed) -> Self {
        let attr = match Self::from_attributes(attrs) {
            Ok(v) => v,
            Err(err) => abort_call_site!("Invalid Attributes: {}", err),
        };

        if attr.parameters.is_some() && !Self::has_parameters_field(fields) {
            abort_call_site!("Missing parameters field")
        }

        attr
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

    fn quote_params<P>(
        &self,
        name: &Ident,
        parameters: &P,
        parameter_map: &'static HashMap<&'static str, (u32, Vec<String>, String, String)>,
    ) -> TokenStream
    where
        P: Getter,
    {
        let fns = parameter_map
            .iter()
            .map(|(k, (v, docs, variant, ty))| match parameters.get(k) {
                false => quote! {},
                true => {
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
                            self.parameters.get(#v).map(|p| match p {
                                crate::types::Parameter::#variant(t) => t,
                                _ => unreachable!(#unreachable_literal)
                            })
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl #name {
                #( #fns )*

                pub fn parameter<T>(&self, key: T) -> Option<&crate::types::Parameter>
                where
                    T: Into<varint::x!(i)>,
                {
                    self.parameters.get(key.into())
                }

                pub fn parameter_mut<T>(&mut self, key: T) -> Option<&mut crate::types::Parameter>
                where
                    T: Into<varint::x!(i)>,
                {
                    self.parameters.get_mut(key.into())
                }
            }
        }
    }

    fn quote_client_setup(&self, name: &Ident) -> TokenStream {
        self.quote_params(
            name,
            &setup::client::Parameters {
                path: true,
                max_request_id: true,
                auth_token: true,
                max_auth_token_cache_size: true,
                authority: true,
                moqt_implementation: true,
            },
            setup::client::parameter_map(),
        )
    }

    fn quote_server_setup(&self, name: &Ident) -> TokenStream {
        self.quote_params(
            name,
            &setup::server::Parameters {
                max_request_id: true,
                auth_token: true,
                max_auth_token_cache_size: true,
                moqt_implementation: true,
            },
            setup::server::parameter_map(),
        )
    }

    fn quote_general(&self, name: &Ident, parameters: &general::Parameters) -> TokenStream {
        self.quote_params(name, parameters, general::parameter_map())
    }

    pub fn quote(&self, name: &Ident) -> TokenStream {
        let Some(parameters) = &self.parameters else {
            return quote! {};
        };

        if name == "ClientSetup" {
            self.quote_client_setup(name)
        } else if name == "ServerSetup" {
            self.quote_server_setup(name)
        } else {
            self.quote_general(name, parameters)
        }
    }
}
