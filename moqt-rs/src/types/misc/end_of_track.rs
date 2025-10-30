use varint::varint_enum;

varint_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(8))]
    pub enum EndOfTrack {
        True = 0x1,
        False = 0x0,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x1, 0x0];

    varint_enum_test!(EndOfTrack; BUF; 0x3F;
        True, False,
    );
}
