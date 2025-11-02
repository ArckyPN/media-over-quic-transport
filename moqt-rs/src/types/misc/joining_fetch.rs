use varint::{VarInt, x};

/// ## Joining Fetch
///
/// Join a Subscribe with a preceding Fetch.
///
/// [Fetch](crate::types::message::Fetch)
#[derive(Debug, VarInt, Clone, PartialEq)]
pub struct JoiningFetch {
    /// ## Request ID
    ///
    /// The associated Request ID of the
    /// Subscribe to join.
    pub request_id: x!(i),

    /// ## Starting Group
    ///
    /// This is either an absolute Group ID
    /// or a relative Group ID, depending on
    /// the Joining Fetch type.
    ///
    /// [FetchType](crate::types::misc::FetchType)
    pub start: x!(i),
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
