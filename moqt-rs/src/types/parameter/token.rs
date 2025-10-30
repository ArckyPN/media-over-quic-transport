use std::fmt::Debug;

use varint::{VarInt, x};

use crate::types::misc::AliasType;

#[derive(VarInt, PartialEq, Clone)]
pub struct Token {
    alias_typ: AliasType,
    #[varint(when(alias_typ = 0x0 || 0x1 || 0x2))]
    alias: x!([i]),
    #[varint(when(alias_typ = 0x1 || 0x3))]
    typ: x!([i]),
    #[varint(when(alias_typ = 0x1 || 0x3))]
    value: x!([..]),
}

impl Token {
    pub fn new_delete<A>(alias: A) -> Self
    where
        A: Into<x!(i)>,
    {
        Self {
            alias_typ: AliasType::Delete,
            alias: Some(alias.into()),
            typ: None,
            value: None,
        }
    }

    pub fn new_register<A, T, V>(alias: A, typ: T, value: V) -> Self
    where
        A: Into<x!(i)>,
        T: Into<x!(i)>,
        V: Into<x!(..)>,
    {
        Self {
            alias_typ: AliasType::Register,
            alias: Some(alias.into()),
            typ: Some(typ.into()),
            value: Some(value.into()),
        }
    }

    pub fn new_use_alias<A>(alias: A) -> Self
    where
        A: Into<x!(i)>,
    {
        Self {
            alias_typ: AliasType::UseAlias,
            alias: Some(alias.into()),
            typ: None,
            value: None,
        }
    }

    pub fn new_use_value<T, V>(typ: T, value: V) -> Self
    where
        T: Into<x!(i)>,
        V: Into<x!(..)>,
    {
        Self {
            alias_typ: AliasType::UseValue,
            alias: None,
            typ: Some(typ.into()),
            value: Some(value.into()),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("alias_typ", &self.alias_typ)
            .field("alias", &self.alias.as_ref().map(|a| a.to_string()))
            .field("type", &self.typ.as_ref().map(|a| a.to_string()))
            .field("value", &self.value)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Token {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new_delete(6u8);
            let b1 = vec![
                0, // delete type
                6, // alias
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::new_register(3u8, 10u8, [1, 2, 3]);
            let b2 = vec![
                1,  // register type
                3,  // alias
                10, // type
                1, 2, 3, // value
            ];
            let l2 = b2.len() * 8;

            let v3 = Self::new_use_alias(8u8);
            let b3 = vec![
                2, // use alias type
                8, // alias
            ];
            let l3 = b3.len() * 8;

            let v4 = Self::new_use_value(40u8, [10, 11, 12, 13]);
            let b4 = vec![
                3,  // use value type
                40, // type
                10, 11, 12, 13, // value
            ];
            let l4 = b4.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3), (v4, b4, l4)]
        }
    }

    varint_struct_test!(Token);
}
