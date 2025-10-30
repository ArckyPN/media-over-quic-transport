use varint::{VarInt, x};

#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct GoAway {
    // TODO not specified but should probably be valid utf-8
    // TODO larger than 8192 bytes => protocol violation
    #[varint(length = x(i))]
    uri: x!(..),
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for GoAway {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                uri: "http:".into(),
            };
            let b1 = [
                vec![5],           // uri length
                b"http:".to_vec(), // uri
            ]
            .concat();
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(GoAway);
}
