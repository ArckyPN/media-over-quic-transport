use varint::varint_enum;

varint_enum! {
    /// Error Code associated with a
    /// [PublishDone](crate::types::message::PublishDone) Control Message.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14)]
    pub enum PublishDone {
        /// An implementation specific or generic
        /// error occurred.
        InternalError = 0x0,

        /// The subscriber is no longer authorized
        /// to subscribe to the given track.
        Unauthorized = 0x1,

        /// The track is no longer being published.
        TrackEnded = 0x2,

        /// The publisher reached the end of an
        /// associated Subscribe filter.
        SubscriptionEnded = 0x3,

        /// The subscriber or publisher issued a
        /// [Goaway](crate::types::message::Goaway) message.
        GoingAway = 0x4,

        /// The publisher reached the timeout
        /// specified in SUBSCRIBE_OK
        Expired = 0x5,

        /// The publisher's queue of objects to
        /// be sent to the given subscriber exceeds
        /// its implementation defined limit.
        TooFarBehind = 0x6,

        /// A relay publisher detected the track
        /// was malformed.
        /// (see Draft[https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#malformed-tracks]).
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
