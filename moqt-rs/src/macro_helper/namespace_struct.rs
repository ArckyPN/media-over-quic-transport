mod sub {
    /// # Namespace Struct Constructor
    ///
    /// Build a public struct which has only one
    /// field of type [Namespace](crate::types::track::Namespace).
    ///
    /// This also generates a bunch of impl block
    /// associated to these types.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// namespace_struct! {
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
    ///     field_ident: Namespace,
    /// }
    /// ```
    macro_rules! namespace_struct {
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
                $field: $crate::types::track::Namespace,
            }

            impl $name {
                pub fn new<N>($field: N) -> Self
                where
                    N: Into<$crate::types::track::Namespace>,
                {
                    Self::from($field)
                }
            }

            impl<T> From<T> for $name
            where
                T: Into<$crate::types::track::Namespace>,
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
                        let v1 = Self::from(["num", "boom"]);
                        let b1 = vec! [
                            2, // 2 element tuple
                            3, // first tuple len 3
                            b'n', b'u', b'm', // tuple "num"
                            4,    // second tuple len 4
                            b'b', b'o', b'o', b'm', // second tuple "boom"
                        ];
                        let l1 = b1.len() * 8;

                        vec! [(v1, b1, l1)]
                    }
                }

                varint_struct_test!($name);
            }
        };
    }
    pub(crate) use namespace_struct;
}

pub(crate) use sub::namespace_struct;
