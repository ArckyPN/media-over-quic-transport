use std::fmt::Display;

use varint::varint_enum;

varint_enum! {
    /// Termination code used when closing a session.
    ///
    /// For QUIC a CONNECTION_CLOSE frame is used
    /// and for WebTransport a CLOSE_WEBTRANSPORT_SESSION
    /// capsule.
    ///
    /// Using this code in a termination is optional.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14)]
    pub enum Termination {
        /// The session is being terminated without an
        /// error.
        NoError = 0x0,

        /// An implementation specific error occurred.
        InternalError = 0x1,

        /// The client is not authorized to establish
        /// a session.
        Unauthorized = 0x2,

        /// The remote endpoint performed an action that
        /// was disallowed by the specification.
        ProtocolViolation = 0x3,

        /// The session was closed because the endpoint
        /// used a Request ID that was smaller than or
        /// equal to a previously received request ID,
        /// or the least- significant bit of the request
        /// ID was incorrect for the endpoint.
        InvalidRequestID = 0x4,

        /// The endpoint attempted to use a Track Alias
        /// that was already in use.
        DuplicateTrackAlias = 0x5,

        /// The key-value pair has a formatting error.
        KeyValueFormattingError = 0x6,

        /// The session was closed because the endpoint
        /// used a Request ID equal to or larger than the current Maximum Request ID.
        TooManyRequests = 0x7,

        /// The PATH parameter was used by a server, on
        /// a WebTransport session, or the server does
        /// not support the path.
        InvalidPath = 0x8,

        /// The PATH parameter does not conform to the
        /// rules in [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#path).
        MalformedPath = 0x9,

        /// The session was closed because the peer took
        /// too long to close the session in response to
        /// a [Goaway](crate::types::message::Goaway)
        /// message. See session migration in
        /// [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#session-migration).
        GoAwayTimeout = 0x10,

        /// The session was closed because the peer took
        /// too long to respond to a control message.
        ControlMessageTimeout = 0x11,

        /// The session was closed because the peer took
        /// too long to send data expected on an open Data
        /// Stream (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#data-streams)).
        /// This includes fields of a stream header or
        /// an object header within a data stream. If
        /// an endpoint times out waiting for a new object
        /// header on an open subgroup stream, it MAY
        /// send a STOP_SENDING on that stream or terminate
        /// the subscription.
        DataStreamTimeout = 0x12,

        /// The Session limit
        /// ([Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#max-auth-token-cache-size))
        /// of the size of all registered Authorization
        /// tokens has been exceeded.
        AuthTokenCacheOverflow = 0x13,

        /// Authorization Token attempted to register an
        /// Alias that was in use
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        DuplicateAuthTokenAlias = 0x14,

        /// The client didn't offer a version supported by
        /// the server.
        VersionNegotiationFailed = 0x15,

        /// Invalid Auth Token serialization during registration
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        MalformedAuthToken = 0x16,

        /// No registered token found for the provided Alias
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        UnknownAuthTokenAlias = 0x17,

        /// Authorization token has expired
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        ExpiredAuthToken = 0x18,

        /// The specified AUTHORITY does not correspond
        /// to this server or cannot be used in this context.
        InvalidAuthority = 0x19,

        /// The AUTHORITY value is syntactically invalid.
        MalformedAuthority = 0x1A,
    }
}

impl Display for Termination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NoError => "no error occurred",
            Self::InternalError => "internal error",
            Self::Unauthorized => "peer was not authorized",
            Self::ProtocolViolation => "MOQT protocol was violated",
            Self::InvalidRequestID => "invalid request ID",
            Self::DuplicateTrackAlias => "duplicate track alias used",
            Self::KeyValueFormattingError => "invalid parameter formatting",
            Self::TooManyRequests => "too many requests",
            Self::InvalidPath => "invalid PATH parameter",
            Self::MalformedPath => "malformed PATH parameter",
            Self::GoAwayTimeout => "peer took too long to follow GOAWAY instruction",
            Self::ControlMessageTimeout => "peer took too long to respond",
            Self::DataStreamTimeout => "peer took too long to send data",
            Self::AuthTokenCacheOverflow => "authorization token cache is full",
            Self::DuplicateAuthTokenAlias => "duplicate authorization token alias",
            Self::VersionNegotiationFailed => "failed negotiate a common version",
            Self::MalformedAuthToken => "malformed authorization token",
            Self::UnknownAuthTokenAlias => "unknown authorization token alias",
            Self::ExpiredAuthToken => "authorization token expired",
            Self::InvalidAuthority => "invalid AUTHORITY parameter",
            Self::MalformedAuthority => "malformed AUTHORITY parameter",
        })
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
