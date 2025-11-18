use {
    crate::types::parameter::ClientSetupParameters,
    bon::Builder,
    varint::{VarInt, x},
};

/// ## Client Setup
///
/// The first Message that is part of the opening
/// handshake to initiate a MOQT Session.
#[derive(Debug, VarInt, PartialEq, Clone, Builder)]
#[varint::draft_ref(v = 14, rename = "client_setup-and-server_set")]
pub struct ClientSetup {
    /// ## Supported Versions
    ///
    /// List of the supported Versions by the Client.
    #[varint(count = x(i))]
    #[builder(field)]
    pub supported_versions: x!(i; ...),
    #[builder(field)]
    pub parameters: ClientSetupParameters,
}

impl<S: client_setup_builder::State> ClientSetupBuilder<S> {
    /// Adds a supported Version to [ClientSetup]
    pub fn version<V>(mut self, v: V) -> Self
    where
        V: Into<x!(i)>,
    {
        self.supported_versions.push(v.into());
        self
    }

    /// Adds supported Versions from an Iterator to [ClientSetup]
    pub fn versions<I, T>(mut self, versions: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<x!(i)>,
    {
        self.supported_versions
            .extend(versions.into_iter().map(Into::into));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for ClientSetup {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder().versions(&[1u8, 2u8]).build();
            let b1 = vec![
                2, // num of supported version
                1, 2, // supported versions
                0, // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::builder()
                .version(1u8)
                .version(2u8)
                .version(3u8)
                .max_request_id(14u8)
                .build();
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
