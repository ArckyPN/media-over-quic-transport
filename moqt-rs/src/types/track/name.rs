use std::fmt::Display;

use varint::{VarInt, VarIntBytes, x};

/// ## Track Name
#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct Name {
    #[varint(length = x(i))]
    inner: x!(..),
}

impl Name {
    pub fn new<T>(num: T) -> Self
    where
        T: Into<x!(..)>,
    {
        Self::from(num)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.inner.bytes()))
    }
}

// TODO impl some stuff to make them more useable, (de)ref to string

impl<T> From<T> for Name
where
    T: Into<x!(..)>,
{
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Name {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new("moqt");
            let b1 = [
                4, // length 4
                b'm', b'o', b'q', b't', // track name "moqt"
            ]
            .to_vec();
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(Name);
}
