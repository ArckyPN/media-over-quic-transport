use {crate::types::ServerSetupParameters, bon::bon, funty::Unsigned, varint::prelude::*};

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
    pub fn selected_version<U>(&self) -> U
    where
        U: Unsigned,
    {
        self.selected_version.number()
    }
}

#[bon]
impl ServerSetup {
    #[builder]
    pub fn new(
        #[builder(field)] parameters: ServerSetupParameters,

        #[builder(into, setters(
        name = version,
        doc {
            /// Sets the selected version on [ServerSetup].
        }
    ))]
        selected_version: x!(i),
    ) -> Self {
        Self {
            selected_version,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for ServerSetup {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder().version(2u8).build();
            let b1 = vec![
                2, // selected version
                0, // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::builder().version(3u8).max_request_id(14u8).build();
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
