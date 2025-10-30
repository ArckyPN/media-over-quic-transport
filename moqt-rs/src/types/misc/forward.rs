use varint::varint_enum;

varint_enum! {
    /// TODO docs
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(8))]
    pub enum Forward {
        /// TODO docs
        Disabled = 0x0,
        /// TODO docs
        Enabled = 0x1,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1];

    varint_enum_test!(Forward; BUF; 0x3F;
        Disabled, Enabled,
    );
}
