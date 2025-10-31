use varint::varint_enum;

varint_enum! {
    /// TODO docs
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, error)]
    pub enum Publish {
        /// TODO docs
        InternalError = 0x0,
        /// TODO docs
        Unauthorized = 0x1,
        /// TODO docs
        Timeout = 0x2,
        /// TODO docs
        NotSupported = 0x3,
        /// TODO docs
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
