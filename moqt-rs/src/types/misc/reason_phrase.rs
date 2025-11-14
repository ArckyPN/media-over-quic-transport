use std::fmt::Display;

use varint::{VarInt, VarIntBytes, draft_ref, x};

/// ## Reason Phrase
#[derive(Debug, VarInt, PartialEq, Clone)]
#[draft_ref(v = 14, rename = "name-reason-phrase-structure")]
pub struct ReasonPhrase {
    // TODO larger than 1024 bytes is Protocol Violation
    // TODO validate it is valid utf8?
    #[varint(length = x(i))]
    value: x!(..),
}

impl ReasonPhrase {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<x!(..)>,
    {
        Self::from(msg)
    }
}

impl<T> From<T> for ReasonPhrase
where
    T: Into<x!(..)>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Display for ReasonPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.value.bytes()))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for ReasonPhrase {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = "error".into();
            let b1 = [vec![5], b"error".to_vec()].concat();
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(ReasonPhrase);
}
