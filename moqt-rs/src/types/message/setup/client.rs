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

impl ClientSetup {
    /// Selects the highest supported version. return None if no versions match.
    ///
    /// # Example
    ///
    /// The idea is that a Relay receives the [ClientSetup] message and
    /// then calls this function to select the negotiated version:
    ///
    /// ```rust,ignore
    /// // server.rs
    /// const SUPPORTED_VERSIONS: &[u32] = &[
    ///     1, 2, 3
    /// ];
    /// let msg: ClientSetup = recv(); // example: msg.supported_version = [1, 2]
    ///
    /// let version = msg.supported_version(SUPPORTED_VERSIONS);
    /// assert_eq!(version, Some(2));
    ///
    /// // send negotiated version to client
    /// let msg = ServerSetup::builder().version(version).build();
    /// send(msg);
    /// ```
    pub fn supported_version<I, T>(&self, server_versions: I) -> Option<x!(i)>
    where
        I: IntoIterator<Item = T>,
        T: Into<x!(i)>,
    {
        let mut selected_version: Option<x!(i)> = None;

        for v in server_versions {
            let server_version = v.into();
            for client_version in &self.supported_versions {
                if server_version == *client_version {
                    match &selected_version {
                        Some(v) => {
                            if *v < server_version {
                                selected_version = Some(server_version.clone())
                            }
                        }
                        None => selected_version = Some(server_version.clone()),
                    }
                }
            }
        }

        selected_version
    }
}

impl<S: client_setup_builder::State> ClientSetupBuilder<S> {
    /// Adds a supported Version to [ClientSetup].
    pub fn version<V>(mut self, v: V) -> Self
    where
        V: Into<x!(i)>,
    {
        self.supported_versions.push(v.into());
        self
    }

    /// Adds supported Versions from an Iterator to [ClientSetup].
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
    use {
        super::*,
        crate::test_helper::{TestData, varint_struct_test},
        pretty_assertions::assert_eq,
    };

    impl TestData for ClientSetup {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder().versions([1u8, 2u8]).build();
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

    #[test]
    fn supported_version_test() {
        let supported_versions = &[1u8, 2u8, 3u8];
        let msg = ClientSetup::builder().versions([1u8, 2u8]).build();
        let valid = msg.supported_version(supported_versions);
        assert_eq!(valid, Some(2u8.into()));

        let supported_versions = &[3u8];
        let msg = ClientSetup::builder().versions([1u8, 2u8]).build();
        let invalid = msg.supported_version(supported_versions);
        assert!(invalid.is_none());
    }
}
