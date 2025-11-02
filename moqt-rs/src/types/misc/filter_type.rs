use varint::varint_enum;

varint_enum! {
    /// ## Filter Type
    ///
    /// Indicates the type of Subscription.
    ///
    /// [Subscribe](crate::types::message::Subscribe)
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum FilterType {
        /// ## NextGroupStart
        ///
        /// The first Object received will be the
        /// first Object of the next published
        /// Group.
        ///
        /// This Subscription is open-ended.
        NextGroupStart = 0x1,

        /// ## LargestObject
        ///
        /// The first Object received will be the
        /// next published Object of the current
        /// Group.
        ///
        /// This Subscription is open-ended.
        LargestObject = 0x2,

        /// ## AbsoluteStart
        ///
        /// The first Object received will be the one
        /// specified by the Subscribe message.
        ///
        /// This Subscription is open-ended.
        AbsoluteStart = 0x3,

        /// ## AbsoluteRange
        ///
        /// The Subscription will be active for the
        /// specified range of Objects.
        AbsoluteRange = 0x4,
    }
}

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
