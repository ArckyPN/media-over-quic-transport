use varint::varint_enum;

varint_enum! {
    /// TODO docs
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(8))]
    pub enum GroupOrder {
        /// TODO docs
        Original = 0x0,
        /// TODO docs
        Ascending = 0x1,
        /// TODO docs
        Descending = 0x2,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1, 0x2];

    varint_enum_test!(GroupOrder; BUF; 0x3F;
        Original, Ascending, Descending,
    );
}
