use varint::varint_enum;

varint_enum! {
    /// TODO docs
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum FilterType {
        /// TODO docs
        NextGroupStart = 0x1,
        /// TODO docs
        LargestObject = 0x2,
        /// TODO docs
        AbsoluteStart = 0x3,
        /// TODO docs
        AbsoluteRange = 0x4,
    }
}
// TODO put into separate file

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x2, 0x1, 0x3, 0x4];

    varint_enum_test!(FilterType; BUF; 0x3F;
        LargestObject, NextGroupStart,
        AbsoluteStart, AbsoluteRange,
    );
}
