use varint::varint_enum;

varint_enum! {
    /// Error Code associated with a
    /// [SubscribeNamespaceError](crate::types::message::SubscribeNamespaceError) Control Message.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, error)]
    pub enum SubscribeNamespace {
        /// An implementation specific or generic
        /// error occurred.
        InternalError = 0x0,

        /// The subscriber is not authorized to
        /// subscribe to the given namespace prefix.
        Unauthorized = 0x1,

        /// The operation could not be completed
        /// before an implementation specific timeout.
        Timeout = 0x2,

        /// The endpoint does not support the
        /// [SubscribeNamespace](crate::types::message::SubscribeNamespace)
        /// method.
        NotSupported = 0x3,

        /// The namespace prefix is not available
        /// for subscription.
        NamespacePrefixUnknown = 0x4,

        /// The namespace prefix overlaps with another
        /// [SubscribeNamespace](crate::types::message::SubscribeNamespace)
        /// in the same session.
        NamespacePrefixOverlap = 0x5,

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

    varint_enum_test!(SubscribeNamespace; BUF; 0x3F;
        InternalError, Unauthorized,
        Timeout, NotSupported,
        NamespacePrefixUnknown, NamespacePrefixOverlap,
        MalformedAuthToken, ExpiredAuthToken,
    );
}
