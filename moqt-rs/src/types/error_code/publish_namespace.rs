use varint::varint_enum;

varint_enum! {
    /// Error Code associated with a
    /// [PublishNamespaceError](crate::types::message::PublishNamespaceError) Control Message.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, error)]
    pub enum PublishNamespace {
        /// An implementation specific or generic
        /// error occurred.
        InternalError = 0x0,

        /// The subscriber is not authorized to
        /// announce the given namespace.
        Unauthorized = 0x1,

        /// The announce could not be completed
        /// before an implementation specific timeout.
        Timeout = 0x2,

        /// The endpoint does not support the
        /// PUBLISH_NAMESPACE method.
        NotSupported = 0x3,

        /// The namespace is not of interest
        /// to the endpoint.
        Uninterested = 0x4,

        /// Invalid Auth Token serialization during
        /// registration
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        MalformedAuthToken = 0x10,

        /// Authorization token has expired (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        ExpiredAuthToken = 0x12,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1, 0x2, 0x3, 0x4, 0x10, 0x12];

    varint_enum_test!(PublishNamespace; BUF; 0x3F;
        InternalError, Unauthorized,
        Timeout, NotSupported,
        Uninterested, MalformedAuthToken,
        ExpiredAuthToken,
    );
}
