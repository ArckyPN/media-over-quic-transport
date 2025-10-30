use varint::varint_enum;

varint_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum FetchType {
        Standalone = 0x1,
        RelativeJoining = 0x2,
        AbsoluteJoining = 0x3,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x1, 0x2, 0x3];

    varint_enum_test!(FetchType; BUF; 0x3F;
        Standalone, RelativeJoining, AbsoluteJoining,
    );
}
