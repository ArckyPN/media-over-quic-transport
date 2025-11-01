use varint::varint_enum;

varint_enum! {
    /// Error Code associated with a
    /// [SubscribeError](crate::types::message::SubscribeError) Control Message.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, error)]
    pub enum Subscribe {
        /// An implementation specific or generic
        /// error occurred.
        InternalError = 0x0,
        /// The subscriber is not authorized to
        /// subscribe to the given track.
        Unauthorized = 0x1,
        /// The subscription could not be completed
        /// before an implementation specific timeout. For example, a relay could not establish an upstream subscription within the timeout.
        Timeout = 0x2,
        /// The endpoint does not support the
        /// SUBSCRIBE method.
        NotSupported = 0x3,
        /// The requested track is not available at
        /// the publisher.
        TrackDoesNotExist = 0x4,
        /// The end of the [Subscribe](crate::types::message::Subscribe)
        /// range is earlier
        /// than the beginning, or the end of the range
        /// has already been published.
        InvalidRange = 0x5,
        /// Invalid Auth Token serialization during
        /// registration
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        MalformedAuthToken = 0x10,
        /// Authorization token has expired
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        ExpiredAuthToken = 0x12,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x10, 0x12];

    varint_enum_test!(Subscribe; BUF; 0x3F;
        InternalError, Unauthorized,
        Timeout, NotSupported,
        TrackDoesNotExist, InvalidRange,
        MalformedAuthToken, ExpiredAuthToken,
    );
}
