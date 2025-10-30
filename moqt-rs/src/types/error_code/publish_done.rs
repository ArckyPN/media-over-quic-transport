use varint::varint_enum;

varint_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum PublishDone {
        InternalError = 0x0,
        Unauthorized = 0x1,
        TrackEnded = 0x2,
        SubscriptionEnded = 0x3,
        GoingAway = 0x4,
        Expired = 0x5,
        TooFarBehind = 0x6,
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
