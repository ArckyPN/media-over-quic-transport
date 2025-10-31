use varint::{VarInt, x};

use crate::types::{error_code, reason_phrase::ReasonPhrase};

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
pub struct PublishDone {
    /// TODO docs
    request_id: x!(i),
    /// TODO docs
    status_code: error_code::PublishDone,
    /// TODO docs
    stream_count: x!(i),
    /// TODO docs
    error_reason: ReasonPhrase,
}

// TODO impls for usability

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for PublishDone {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                request_id: 9u8.into(),
                status_code: error_code::PublishDone::GoingAway,
                stream_count: 15u8.into(),
                error_reason: "stop".into(),
            };
            let b1 = [
                [
                    9,  // ID 9
                    4,  // going away
                    15, // 15 streams
                    4,  // phrase len
                ]
                .to_vec(),
                b"stop".to_vec(),
            ]
            .concat();
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(PublishDone);
}
