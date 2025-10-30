use varint::varint_enum;

varint_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum Termination {
        NoError = 0x0,
        InternalError = 0x1,
        Unauthorized = 0x2,
        ProtocolViolation = 0x3,
        InvalidRequestID = 0x4,
        DuplicateTrackAlias = 0x5,
        KeyValueFormattingError = 0x6,
        TooManyRequests = 0x7,
        InvalidPath = 0x8,
        MalformedPath = 0x9,
        GoAwayTimeout = 0x10,
        ControlMessageTimeout = 0x11,
        DataStreamTimeout = 0x12,
        AuthTokenCacheOverflow = 0x13,
        DuplicateAuthTokenAlias = 0x14,
        VersionNegotiationFailed = 0x15,
        MalformedAuthToken = 0x16,
        UnknownAuthTokenAlias = 0x17,
        ExpiredAuthToken = 0x18,
        InvalidAuthority = 0x19,
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
