use varint::varint_enum;

varint_enum! {
    /// ## Group Order
    ///
    /// Defines in which order Groups are
    /// transmitted.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(8))]
    pub enum GroupOrder {
        /// ## Original Order
        ///
        /// The order as intended by the Publisher.
        Original = 0x0,

        /// ## Ascending Order
        ///
        /// In ascending order of Group IDs.
        Ascending = 0x1,

        /// ## Descending Order
        ///
        /// In descending order of Group IDs.
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
