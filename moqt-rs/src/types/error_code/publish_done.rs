use varint::varint_enum;

varint_enum! {
    /// TODO docs
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14)]
    pub enum PublishDone {
        /// TODO docs
        InternalError = 0x0,
        /// TODO docs
        Unauthorized = 0x1,
        /// TODO docs
        TrackEnded = 0x2,
        /// TODO docs
        SubscriptionEnded = 0x3,
        /// TODO docs
        GoingAway = 0x4,
        /// TODO docs
        Expired = 0x5,
        /// TODO docs
        TooFarBehind = 0x6,
        /// TODO docs
        MalformedTrack = 0x7,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7];

    varint_enum_test!(PublishDone; BUF; 0x3F;
        InternalError, Unauthorized,
        TrackEnded, SubscriptionEnded,
        GoingAway, Expired,
        TooFarBehind, MalformedTrack,
    );
}
