use varint::varint_enum;

varint_enum! {
    /// Error Code associated with a
    /// [PublishError](crate::types::message::PublishError) Control Message.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, error)]
    pub enum Publish {
        /// An implementation specific or generic
        /// error occurred.
        InternalError = 0x0,
        /// The publisher is not authorized to
        /// publish the given namespace or track.
        Unauthorized = 0x1,
        /// The subscription could not be established
        /// before an implementation specific timeout.
        Timeout = 0x2,
        /// The endpoint does not support the
        /// PUBLISH method.
        NotSupported = 0x3,
        /// The namespace or track is not of
        /// interest to the endpoint.
        Uninterested = 0x4,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1, 0x2, 0x3, 0x4];

    varint_enum_test!(Publish; BUF; 0x3F;
        InternalError, Unauthorized,
        Timeout, NotSupported,
        Uninterested,
    );
}
