use bon::Builder;
use varint::{VarInt, x};

/// ## Joining Fetch
///
/// Join a Subscribe with a preceding Fetch.
///
/// [Fetch](crate::types::message::Fetch)
#[derive(Debug, VarInt, Clone, PartialEq, Builder)]
pub struct JoiningFetch {
    /// ## Request ID
    ///
    /// The associated Request ID of the
    /// Subscribe to join.
    #[builder(into, setters(
        name = id,
        doc {
            /// Sets the request ID on [JoiningFetch].
        }
    ))]
    pub request_id: x!(i),

    /// ## Starting Group
    ///
    /// This is either an absolute Group ID
    /// or a relative Group ID, depending on
    /// the Joining Fetch type.
    ///
    /// [FetchType](crate::types::misc::FetchType)
    #[builder(into, setters(
        doc {
            /// Sets the start group on [JoiningFetch].
        }
    ))]
    pub start: x!(i),
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for JoiningFetch {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder().id(5u8).start(0u8).build();
            let b1 = vec![5, 0];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(JoiningFetch);
}
