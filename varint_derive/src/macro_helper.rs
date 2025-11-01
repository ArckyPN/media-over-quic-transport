mod sub {
    macro_rules! ty_vec {
        (
            $($ty:ty),* $(,)?
        ) => {
            vec![
                $(
                    quote::quote! { $ty }
                ),*
            ]
        };
    }

    macro_rules! parameter_map {
        (
            $(
                $(#[$attrs:meta])*
                $names:ident = $keys:literal => $variant:ident as $tys:path
            ),* $(,)?
        ) => {
            pub fn parameter_map() -> &'static std::collections::HashMap<&'static str, (u32, Vec<String>, String, String)> {
                static HASH_MAP: std::sync::OnceLock<std::collections::HashMap<&str, (u32, Vec<String>, String, String)>> = std::sync::OnceLock::new();
                HASH_MAP.get_or_init(|| {
                    std::collections::HashMap::from([
                        $(
                            (
                                stringify!($names),
                                (
                                    $keys,
                                    vec![
                                        $(
                                            stringify!($attrs).to_string()
                                        ),*
                                    ],
                                    stringify!($variant).to_string(),
                                    stringify!($tys).to_string(),
                                )
                            )
                        ),*
                    ])
                })
            }

            #[derive(darling::FromMeta)]
            pub struct Parameters {
                $(
                    #[darling(default)]
                    pub(crate) $names: bool,
                )*
            }

            impl $crate::structs::Getter for Parameters {
                fn get(&self, s: &str) -> bool {
                    match s {
                        $(
                            stringify!($names) => self.$names,
                        )*
                        _ => unreachable!("you should use the key of the parameter_map() as accessors")
                    }
                }
            }
        };
    }

    pub(crate) use {parameter_map, ty_vec};
}

pub(crate) use sub::{parameter_map, ty_vec};
