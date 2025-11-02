use varint::varint_enum;

varint_enum! {
    /// Error Code associated with a
    /// [FetchError](crate::types::message::FetchError) Control Message.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint::draft_ref(v = 14, error)]
    pub enum Fetch {
        /// An implementation specific or generic
        /// error occurred.
        InternalError = 0x0,

        /// The subscriber is not authorized to
        /// fetch from the given track.
        Unauthorized = 0x1,

        /// The fetch could not be completed before
        /// an implementation specific timeout. For
        /// example, a relay could not FETCH missing
        /// objects within the timeout.
        Timeout = 0x2,

        /// The endpoint does not support the FETCH
        /// method.
        NotSupported = 0x3,

        /// The requested track is not available at the
        /// publisher.
        TrackDoesNotExist = 0x4,

        /// The end of the requested range is earlier
        /// than the beginning, the start of the requested
        /// range is beyond the Largest Location, or the
        /// track has not published any Objects yet.
        InvalidRange = 0x5,

        /// No Objects exist between the requested Start
        /// and End Locations.
        NoObjects = 0x6,

        /// The joining Fetch referenced a Request ID that
        /// did not belong to an active Subscription.
        InvalidJoiningRequestId = 0x7,

        /// The requested range contains objects with
        /// unknown status.
        UnknownStatusInRange = 0x8,

        /// A relay publisher detected the track was malformed
        /// (see Draft[https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#malformed-tracks]).
        MalformedTrack = 0x9,

        /// Invalid Auth Token serialization during
        /// registration
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        MalformedAuthToken = 0x10,

        /// Authorization token has expired
        /// (see [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#authorization-token)).
        ExpiredAuthToken = 0x12,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0x10, 0x12];

    varint_enum_test!(Fetch; BUF; 0x3F;
        InternalError, Unauthorized,
        Timeout, NotSupported,
        TrackDoesNotExist, InvalidRange,
        NoObjects, InvalidJoiningRequestId,
        UnknownStatusInRange, MalformedTrack,
        MalformedAuthToken, ExpiredAuthToken,
    );
}
