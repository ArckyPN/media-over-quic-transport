mod error;
mod ok;
mod un;

pub use error::SubscribeNamespaceError;
pub use ok::SubscribeNamespaceOk;
pub use un::UnsubscribeNamespace;

use varint::{VarInt, x};

use crate::types::track;

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
pub struct SubscribeNamespace {
    /// TODO docs
    request_id: x!(i),
    /// TODO docs
    namespace_prefix: track::Namespace,
    // TODO parameters
}

impl SubscribeNamespace {
    pub fn new<ID, N>(id: ID, namespace: N) -> Self
    where
        ID: Into<x!(i)>,
        N: Into<track::Namespace>,
    {
        Self {
            request_id: id.into(),
            namespace_prefix: namespace.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for SubscribeNamespace {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(15u8, ["num", "boom"]);
            let b1 = vec![
                15, // request id: 15
                2,  // 2 element tuple
                3,  // first tuple len 3
                b'n', b'u', b'm', // tuple "num"
                4,    // second tuple len 4
                b'b', b'o', b'o',
                b'm', // second tuple "boom"
                      // TODO parameters
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(SubscribeNamespace);
}
