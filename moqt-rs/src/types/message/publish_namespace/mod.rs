mod cancel;
mod done;
mod error;
mod ok;

pub use {
    cancel::PublishNamespaceCancel, done::PublishNamespaceDone, error::PublishNamespaceError,
    ok::PublishNamespaceOk,
};

use {
    crate::types::{Parameters, track::Namespace},
    bon::bon,
    varint::{VarInt, x},
};

/// ## PublishNamespace
///
/// A Publisher advertises that it has Tracks
/// available on the given Namespace.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token))]
pub struct PublishNamespace {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Track Namespace
    ///
    /// The advertised Namespace.
    ///
    /// [Namespace]
    pub namespace: Namespace,

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

#[bon]
impl PublishNamespace {
    #[builder]
    pub fn new(
        #[builder(field)] parameters: Parameters,
        #[builder(into, setters(
            name = id,
            doc {
                /// TODO docs
            }
        ))]
        request_id: x!(i),
        #[builder(into, setters(
            doc {
                /// TODO docs
            }
        ))]
        namespace: Namespace,
    ) -> Self {
        Self {
            request_id,
            namespace,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for PublishNamespace {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder().id(3u8).namespace(["num", "boom"]).build();
            let b1 = vec![
                3, // request id: 3
                2, // 2 element tuple
                3, // first tuple len 3
                b'n', b'u', b'm', // tuple "num"
                4,    // second tuple len 4
                b'b', b'o', b'o', b'm', // second tuple "boom"
                0,    // no parameters
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }
    varint_struct_test!(PublishNamespace);
}
