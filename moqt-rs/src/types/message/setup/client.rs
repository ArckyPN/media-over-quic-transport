use {
    crate::types::parameter::ClientSetupParameters,
    varint::{VarInt, x},
};

/// ## Client Setup
///
/// The first Message that is part of the opening
/// handshake to initiate a MOQT Session.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14, rename = "client_setup-and-server_set")]
pub struct ClientSetup {
    /// ## Supported Versions
    ///
    /// List of the supported Versions by the Client.
    #[varint(count = x(i))]
    pub supported_versions: x!(i; ...),
    pub parameters: ClientSetupParameters,
}

impl ClientSetup {
    pub fn new<V, P>(versions: &[V], params: P) -> Self
    where
        V: Into<x!(i)> + Clone,
        P: Into<ClientSetupParameters>,
    {
        Self {
            supported_versions: Vec::from_iter(versions.iter().map(|v| v.clone().into())),
            parameters: params.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_helper::{TestData, varint_struct_test},
        types::ClientSetupParameter,
    };

    use super::*;

    impl TestData for ClientSetup {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(&[1u8, 2u8], []);
            let b1 = vec![
                2, // num of supported version
                1, 2, // supported versions
                0, // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::new(
                &[1u8, 2u8, 3u8],
                [(0x02u8.into(), ClientSetupParameter::MaxRequestId(14))],
            );
            let b2 = vec![
                3, // num of supported version
                1, 2, 3,    // supported versions
                1,    // 1 parameter
                0x02, // MaxRequestId param
                14,   // param value
            ];
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(ClientSetup);
}
