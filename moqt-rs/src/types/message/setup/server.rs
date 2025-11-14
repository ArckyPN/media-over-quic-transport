use varint::prelude::*;

use crate::types::ServerSetupParameters;

/// ## ServerSetup
///
/// The Response to the opening handshake
/// Message [ClientSetup](crate::types::message::ClientSetup).
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14, rename = "client_setup-and-server_set")]
pub struct ServerSetup {
    /// ## Selected Version
    ///
    /// The selected Version chosen by the
    /// Server.
    selected_version: x!(i),

    parameters: ServerSetupParameters,
}

impl ServerSetup {
    pub fn new<V, P>(version: V, params: P) -> Self
    where
        V: Into<x!(i)>,
        P: Into<ServerSetupParameters>,
    {
        Self {
            selected_version: version.into(),
            parameters: params.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_helper::{TestData, varint_struct_test},
        types::ServerSetupParameter,
    };

    use super::*;

    impl TestData for ServerSetup {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(2u8, []);
            let b1 = vec![
                2, // selected version
                0, // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::new(
                3u8,
                [(0x02u8.into(), ServerSetupParameter::MaxRequestId(14))],
            );
            let b2 = vec![
                3,    // selected version
                1,    // 1 parameter
                0x02, // MaxRequestId param
                14,   // param value
            ];
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(ServerSetup);
}
