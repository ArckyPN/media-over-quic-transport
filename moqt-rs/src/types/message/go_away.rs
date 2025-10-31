use varint::{VarInt, x};

/// TODO docs
///
/// some placeholder stuff
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
pub struct Goaway {
    /// TODO docs
    // TODO not specified but should probably be valid utf-8
    // TODO larger than 8192 bytes => protocol violation
    #[varint(length = x(i))]
    uri: x!(..),
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Goaway {
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

    varint_struct_test!(Goaway);
}
