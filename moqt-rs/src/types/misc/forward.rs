use varint::varint_enum;

varint_enum! {
    /// ## Forward Toggle
    ///
    /// Indicates whether or not Objects are
    /// forwarded.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(8))]
    pub enum Forward {
        /// ## Forward Disabled
        ///
        /// Objects are not to be forwarded.
        Disabled = 0x0,

        /// ## Forward Enabled
        ///
        /// Objects are to be forwarded.
        Enabled = 0x1,
    }
}

impl From<bool> for Forward {
    fn from(value: bool) -> Self {
        if value { Self::Enabled } else { Self::Disabled }
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
