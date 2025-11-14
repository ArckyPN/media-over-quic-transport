use {
    super::Token,
    crate::macro_helper::parameter_enum,
    std::{fmt::Debug, time::Duration},
    varint::prelude::*,
};

parameter_enum! {
    /// ## Version Specific Parameters
    #[derive(PartialEq, Clone)]
    pub enum Parameter {
        /// ## Delivery Timeout
        DeliveryTimeout(Duration) = 0x02 -> Number,

        /// ## Authorization Token
        ///
        /// is repeatable
        AuthorizationToken(Token) = 0x03 -> Bytes,

        /// ## Max Cache Duration
        ///
        MaxCacheDuration(Duration) = 0x04 -> Number,
    }
}

impl Debug for Parameter {
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
            Self::DeliveryTimeout(d) => f
                .debug_tuple("Parameter::DeliveryTimeout")
                .field(d)
                .finish(),
            Self::AuthorizationToken(t) => f.debug_tuple("Parameter::AuthToken").field(t).finish(),
            Self::MaxCacheDuration(d) => f
                .debug_tuple("Parameter::MaxCacheDuration")
                .field(d)
                .finish(),
        }
    }
}
