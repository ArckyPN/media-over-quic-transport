use varint::varint_enum;

varint_enum! {
    /// TODO docs
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14)]
    pub enum Termination {
        /// TODO docs
        NoError = 0x0,
        /// TODO docs
        InternalError = 0x1,
        /// TODO docs
        Unauthorized = 0x2,
        /// TODO docs
        ProtocolViolation = 0x3,
        /// TODO docs
        InvalidRequestID = 0x4,
        /// TODO docs
        DuplicateTrackAlias = 0x5,
        /// TODO docs
        KeyValueFormattingError = 0x6,
        /// TODO docs
        TooManyRequests = 0x7,
        /// TODO docs
        InvalidPath = 0x8,
        /// TODO docs
        MalformedPath = 0x9,
        /// TODO docs
        GoAwayTimeout = 0x10,
        /// TODO docs
        ControlMessageTimeout = 0x11,
        /// TODO docs
        DataStreamTimeout = 0x12,
        /// TODO docs
        AuthTokenCacheOverflow = 0x13,
        /// TODO docs
        DuplicateAuthTokenAlias = 0x14,
        /// TODO docs
        VersionNegotiationFailed = 0x15,
        /// TODO docs
        MalformedAuthToken = 0x16,
        /// TODO docs
        UnknownAuthTokenAlias = 0x17,
        /// TODO docs
        ExpiredAuthToken = 0x18,
        /// TODO docs
        InvalidAuthority = 0x19,
        /// TODO docs
        MalformedAuthority = 0x1A,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
        0x17, 0x18, 0x19, 0x1A,
    ];

    varint_enum_test!(Termination; BUF; 0x3F;
        NoError, InternalError,
        Unauthorized, ProtocolViolation,
        InvalidRequestID, DuplicateTrackAlias,
        KeyValueFormattingError, TooManyRequests,
        InvalidPath, MalformedPath,
        GoAwayTimeout, ControlMessageTimeout,
        DataStreamTimeout, AuthTokenCacheOverflow,
        DuplicateAuthTokenAlias, VersionNegotiationFailed,
        MalformedAuthToken, UnknownAuthTokenAlias,
        ExpiredAuthToken, InvalidAuthority,
        MalformedAuthority,
    );
}
