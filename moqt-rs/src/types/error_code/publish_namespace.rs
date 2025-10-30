use varint::varint_enum;

varint_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum PublishNamespace {
        InternalError = 0x0,
        Unauthorized = 0x1,
        Timeout = 0x2,
        NotSupported = 0x3,
        Uninterested = 0x4,
        MalformedAuthToken = 0x10,
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
