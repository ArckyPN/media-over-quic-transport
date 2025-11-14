use {
    crate::{macro_helper::parameter_enum, types::Token},
    std::fmt::Debug,
    varint::VarIntBytes,
};

parameter_enum! {
    /// ## Client Setup Parameters
    #[derive(PartialEq, Clone)]
    pub enum ServerSetupParameter {
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

        /// Maximum size if bytes of all active
        /// Authorization tokens per session.
        ///
        /// Default is 0. If not set, Auth Token
        /// Aliases are prohibited.
        MaxAuthorizationTokenCacheSize(u64) = 0x04 -> Number,

        /// Signals Name and Version of the supported
        /// MOQT Implementation.
        MoqtImplemenation(String) = 0x07 -> Bytes, // TODO draft says its key 0x05, same as Authority?
    }
}

impl Debug for ServerSetupParameter {
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
            Self::MaxRequestId(d) => f.debug_tuple("Parameter::MaxRequestId").field(d).finish(),
            Self::AuthorizationToken(t) => f.debug_tuple("Parameter::AuthToken").field(t).finish(),
            Self::MaxAuthorizationTokenCacheSize(n) => f
                .debug_tuple("Parameter::MaxAuthorizationTokenCacheSize")
                .field(n)
                .finish(),
            Self::MoqtImplemenation(t) => f
                .debug_tuple("Parameter::MoqtImplemenation")
                .field(t)
                .finish(),
        }
    }
}
