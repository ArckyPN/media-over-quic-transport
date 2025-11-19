mod error;
mod ok;
mod un;

pub use {error::SubscribeNamespaceError, ok::SubscribeNamespaceOk, un::UnsubscribeNamespace};

use {
    crate::types::{Parameters, track::Namespace},
    bon::bon,
    varint::{VarInt, x},
};

/// ## SubscribeNamespace
///
/// Subscribe to a Namespace Prefix to receive
/// all matching Namespaces, as well as future
/// updates to them.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token))]
pub struct SubscribeNamespace {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Track Namespace Prefix
    ///
    /// The requested Namespace.
    ///
    /// [Namespace]
    pub namespace_prefix: Namespace,

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

#[bon]
impl SubscribeNamespace {
    #[builder]
    pub fn new(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [SubscribeNamespace].
            }
        ))]
        request_id: x!(i),
        #[builder(into, setters(
            doc {
                /// Sets the track namespace prefix on [SubscribeNamespace].
            }
        ))]
        namespace_prefix: Namespace,
    ) -> Self {
        Self {
            request_id,
            namespace_prefix,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for SubscribeNamespace {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .id(15u8)
                .namespace_prefix(["num", "boom"])
                .build();
            let b1 = vec![
                15, // request id: 15
                2,  // 2 element tuple
                3,  // first tuple len 3
                b'n', b'u', b'm', // tuple "num"
                4,    // second tuple len 4
                b'b', b'o', b'o', b'm', // second tuple "boom"
                0,    // no parameters
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(SubscribeNamespace);
}
