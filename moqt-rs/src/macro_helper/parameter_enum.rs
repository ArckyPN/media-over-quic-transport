mod sub {
    /// Creates a Parameter Enum, which will be valid
    /// to used as Key in a Parameter Map: [IndexMap](indexmap::IndexMap)<[x(i)](varint::core::Number), `here`>
    ///
    /// # Syntax
    ///
    /// ```ignore
    /// parameter_enum! {
    ///     /// optional attributes
    ///     pub enum EnumName {
    ///         /// optional attributes (for each Variant)
    ///         // Important: TypePath1 must implement TryFrom<varint::x!(i)>!
    ///         VariantName1(TypePath1) = `even uint_literal` -> Number,
    ///         // Important: TypePath2 must implement TryFrom<varint::x!(..)>!
    ///         VarintName2(TypePath2) = `odd uint_literal` -> Bytes
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```ignore
    /// parameter_enum! {
    ///     /// Parameter Types
    ///     #[derive(Debug, Clone)]
    ///     pub enum Parameter {
    ///         /// some duration param
    ///         Timeout(Duration) = 0x02 -> Number,
    ///         /// Some data param
    ///         SomeData(DataStruct) = 0x05 -> Bytes,
    ///     }
    /// }
    /// ```
    ///
    /// This will generate:
    ///
    /// ```ignore
    /// /// Parameter Types
    /// #[derive(Debug, Clone)]
    /// pub enum Parameter {
    ///     /// ## Key: `0x02`
    ///     ///
    ///     /// some duration param
    ///     Timeout(Duration),
    ///
    ///     /// ## Key: `0x05`
    ///     ///
    ///     /// Some data param
    ///     SomeData(DataStruct),
    ///
    ///     /// ## Key: `even`
    ///     ///
    ///     /// Generic Number parameter
    ///     Number(varint::x!(i)),
    ///
    ///     /// ## Key: `odd`
    ///     ///
    ///     /// Generic Bytes parameter
    ///     Bytes(varint::x!(..)),
    /// }
    ///
    /// #[derive(Debug, snafu::Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
    /// pub enum ParameterError {
    ///     /// Error when TryFrom failed to parse predefined Parameters
    ///     #[snafu(display("failed to create type from other"))]
    ///     TryFrom,
    ///
    ///     /// Error when an even Key has a Byte Value or an odd Key
    ///     /// has a Number Value
    ///     #[snafu(display("mismatched key-value encoding"))]
    ///     InvalidEncoding,
    /// }
    ///
    /// impl TryFrom<varint::core::external_impls::KeyValuePair> for Parameter {
    ///     /* ... */
    /// }
    ///
    /// impl varint::Parameter for Parameter {
    ///     /* ... */
    /// }
    /// ```
    macro_rules! parameter_enum {
        (
            $(#[$attrss:meta])*
            $vis:vis enum $name:ident {
                $(
                    $(#[$attrs:meta])*
                    $variants:ident($ty:path) = $values:literal -> $kvp:ident
                ),*
                $(,)?
            }
        ) => { paste::paste! {
            $(#[$attrss])*
            $vis enum $name {
                $(
                    #[doc = "## Key: `" $values "`\n\n"]
                    $(#[$attrs])*
                    $variants($ty),
                )*

                /// ## Key: `even`
                ///
                /// Generic Number parameter
                Number(varint::x!(i)),

                /// ## Key: `odd`
                ///
                /// Generic Bytes parameter
                Bytes(varint::x!(..)),
            }

            /// TODO doc
            #[derive(Debug, snafu::Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
            $vis enum [< $name Error >] {
                /// Error when TryFrom failed to parse predefined Parameters
                #[snafu(display("failed to create type from other"))]
                TryFrom,

                /// Error when an even Key has a Byte Value or an odd Key
                /// has a Number Value
                #[snafu(display("mismatched key-value encoding"))]
                InvalidEncoding,
            }

            impl TryFrom<varint::core::external_impls::KeyValuePair> for $name {
                type Error = [< $name Error >];
                fn try_from(value: varint::core::external_impls::KeyValuePair) -> Result<Self, Self::Error> {
                    Ok(match value {
                        $(
                            varint::core::external_impls::KeyValuePair {
                                key, value: varint::core::external_impls::Value::$kvp(v)
                            } if key == $values => Self::$variants($ty::try_from(v).or(Err([< $name Error >]::TryFrom))?),
                        )*
                        varint::core::external_impls::KeyValuePair {
                            value: varint::core::external_impls::Value::Number(v), ..
                        } => Self::Number(v),
                        varint::core::external_impls::KeyValuePair {
                            value: varint::core::external_impls::Value::Bytes(v), ..
                        } => Self::Bytes(v),
                    })
                }
            }

            impl varint::Parameter for $name {
                type PError = [< $name Error >];

                fn to_kvp(&self, key: varint::x!(i)) -> Result<varint::core::external_impls::KeyValuePair, Self::PError> {
                    match (varint::VarIntNumber::number::<u64>(&key), self) {
                        $(
                            (x, Self::$variants(v)) if x == $values =>
                            Ok(varint::core::external_impls::KeyValuePair {
                                key, value: varint::core::external_impls::Value::$kvp(v.clone().try_into().unwrap())
                            }),
                        )*
                        (x, Self::Number(v)) if x.is_multiple_of(2) => Ok(varint::core::external_impls::KeyValuePair {
                            key, value: varint::core::external_impls::Value::Number(v.clone())
                        }),
                        (x, Self::Bytes(v)) if !x.is_multiple_of(2) => Ok(varint::core::external_impls::KeyValuePair {
                            key, value: varint::core::external_impls::Value::Bytes(v.clone())
                        }),
                        _ => Err([< $name Error >]::InvalidEncoding)
                    }
                }
            }
        }};
    }

    pub(crate) use parameter_enum;
}

pub(crate) use sub::parameter_enum;
