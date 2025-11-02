mod cancel;
mod done;
mod error;
mod ok;

pub use cancel::PublishNamespaceCancel;
pub use done::PublishNamespaceDone;
pub use error::PublishNamespaceError;
pub use ok::PublishNamespaceOk;

use varint::{VarInt, x};

use crate::types::{Parameters, track};

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
    /// [Namespace](track::Namespace)
    pub namespace: track::Namespace,

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

impl PublishNamespace {
    pub fn new<ID, N>(id: ID, namespace: N, params: Option<Parameters>) -> Self
    where
        ID: Into<x!(i)>,
        N: Into<track::Namespace>,
    {
        Self {
            request_id: id.into(),
            namespace: namespace.into(),
            parameters: params.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for PublishNamespace {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(3u8, ["num", "boom"], None);
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
