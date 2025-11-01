mod sub {
    /// # VarInt Number Struct Constructor
    ///
    /// Build a public struct which has only one
    /// field of type [Number](varint::core::Number)
    /// aka. `x(i)`.
    ///
    /// This also generates a bunch of impl block
    /// associated to these types.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// number_struct! {
    ///     /// all attributes are passed through to the struct
    ///     StructIdent
    ///     /// all attributes are passed through to the field
    ///     field_ident
    /// }
    /// ```
    ///
    /// ## Generated Code
    ///
    /// ```rust,ignore
    /// /// all attributes are passed through to the struct
    /// pub struct StructIdent {
    ///     /// all attributes are passed through to the field
    ///     field_ident: varint::x!(i),
    /// }
    /// ```
    macro_rules! number_struct {
        (
            $(#[$attrss:meta])*
            $name:ident
            $(#[$attrs:meta])*
            $field:ident
        ) => {
            use varint::VarInt;

            $(#[$attrss])*
            #[derive(Debug, VarInt, PartialEq, Clone)]
            pub struct $name {
                $(#[$attrs])*
                $field: varint::x!(i),
            }

            impl $name {
                pub fn new<ID>(id: ID) -> Self
                where
                    ID: Into<varint::x!(i)>,
                {
                    Self::from(id)
                }
            }

            impl<T> From<T> for $name
            where
                T: Into<varint::x!(i)>,
            {
                fn from(value: T) -> Self {
                    Self {
                        $field: value.into(),
                    }
                }
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                use crate::test_helper::{TestData, varint_struct_test};

                impl TestData for $name {
                    fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
                        let v1 = Self::from(45u8);
                        let b1 = vec! [
                            45, // id
                        ];
                        let l1 = b1.len() * 8;

                        vec! [(v1, b1, l1)]
                    }
                }

                varint_struct_test!($name);
            }
        };
    }
    pub(crate) use number_struct;
}

pub(crate) use sub::number_struct;
