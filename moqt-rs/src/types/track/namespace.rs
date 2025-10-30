use varint::{VarInt, x};

const MIN_LEN: usize = 1;
const MAX_LEN: usize = 32;

#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct Namespace {
    // TODO how to make sure there are only 1..=32 Tuples?
    // TODO total length should also not exceed 4096 bytes
    // both cases => ProtocolViolation
    inner: x!(tuple),
}

impl Namespace {
    pub fn new<T>(tup: T) -> Self
    where
        T: Into<x!(tuple)>,
    {
        Self::from(tup)
    }
}

// TODO impl some stuff to make them more useable, IntoIterator, IntoIterator<'a>, Deref, see Tuple and BinaryData!

impl<T> From<T> for Namespace
where
    T: Into<x!(tuple)>,
{
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_helper::{TestData, varint_struct_test};

    impl TestData for Namespace {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::from(&["moq", "vod", "banana"]);
            let b1 = [
                [
                    3, // len 3
                    3, // len of first name
                ]
                .to_vec(),
                b"moq".to_vec(),
                [
                    3, // len of second name
                ]
                .to_vec(),
                b"vod".to_vec(),
                [
                    6, // len of third name
                ]
                .to_vec(),
                b"banana".to_vec(),
            ]
            .concat();
            let l1 = b1.len() * 8;

            let v2 = Self::from(["test.com"]);
            let b2 = [[1, 8].to_vec(), b"test.com".to_vec()].concat();
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(Namespace);
}
