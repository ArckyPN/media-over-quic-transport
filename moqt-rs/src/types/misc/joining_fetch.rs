use varint::{VarInt, x};

#[derive(Debug, VarInt, Clone, PartialEq)]
pub struct JoiningFetch {
    request_id: x!(i),
    start: x!(i),
}

impl JoiningFetch {
    pub fn new<I, S>(request_id: I, start: S) -> Self
    where
        I: Into<x!(i)>,
        S: Into<x!(i)>,
    {
        Self {
            request_id: request_id.into(),
            start: start.into(),
        }
    }

    // TODO try new with TryInto
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for JoiningFetch {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(5u8, 0u8);
            let b1 = vec![5, 0];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(JoiningFetch);
}
