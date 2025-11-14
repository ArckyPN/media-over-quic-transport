use {super::Token, crate::macro_helper::parameter_enum, std::fmt::Debug, varint::VarIntBytes};

parameter_enum! {
    /// ## Client Setup Parameters
    #[derive(PartialEq, Clone)]
    pub enum ClientSetupParameter {
        /// Path of the MoQ URI
        ///
        /// Must only be used with native QUIC!
        // TODO use rfc3986 uri instead of String: https://docs.rs/percent-encoding-rfc3986/latest/percent_encoding_rfc3986/
        Path(String) = 0x01 -> Bytes,

        /// Initial Value of the largest allowed
        /// request ID.
        ///
        /// Default is 0. If not set, the peer
        /// is not allowed to send any requests.
        MaxRequestId(u64) = 0x02 -> Number,

        /// ## Authorization Token
        ///
        /// is repeatable
        AuthorizationToken(Token) = 0x03 -> Bytes,

        /// Authority URI
        ///
        /// Must only be used with native QUIC!
        // TODO use rfc3986 uri instead of String: https://docs.rs/percent-encoding-rfc3986/latest/percent_encoding_rfc3986/
        Authority(String) = 0x05 -> Bytes,

        /// Signals Name and Version of the supported
        /// MOQT Implementation.
        MoqtImplemenation(String) = 0x07 -> Bytes, // TODO draft says its key 0x05, same as Authority?
    }
}

impl Debug for ClientSetupParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => f
                .debug_tuple("Parameter::Number")
                .field(&n.to_string())
                .finish(),
            Self::Bytes(b) => f
                .debug_struct("Parameter::Bytes")
                .field("buffer", b)
                .field("string", &String::from_utf8_lossy(&b.bytes()))
                .finish(),
            Self::Path(p) => f.debug_tuple("Parameter::Path").field(p).finish(),
            Self::MaxRequestId(d) => f.debug_tuple("Parameter::MaxRequestId").field(d).finish(),
            Self::AuthorizationToken(t) => f.debug_tuple("Parameter::AuthToken").field(t).finish(),
            Self::Authority(p) => f.debug_tuple("Parameter::Authority").field(p).finish(),
            Self::MoqtImplemenation(t) => f
                .debug_tuple("Parameter::MoqtImplemenation")
                .field(t)
                .finish(),
        }
    }
}
