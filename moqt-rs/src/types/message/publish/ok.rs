use varint::{VarInt, x};

use crate::types::{
    Parameters,
    misc::{FilterType, Forward, GroupOrder, Location},
};

/// ## PublishOk
///
/// Response to a successful [Publish](crate::types::message::Publish).
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(delivery_timeout))]
pub struct PublishOk {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Forward Mode
    ///
    /// [Forward]
    pub forward: Forward,

    /// ## Subscriber Priority
    ///
    /// Sets a priority in relation to all Fetches
    /// and Subscribes in the current Session.
    ///
    /// Lower means higher priority.
    pub subscriber_priority: x!(8),

    /// ## Group Order
    ///
    /// The Order in which Groups will be published.
    ///
    /// [GroupOrder]
    pub group_order: GroupOrder,

    /// ## Filter Type
    ///
    /// Indicates the Publish mode.
    ///
    /// [FilterType]
    pub filter_type: FilterType,

    /// ## First Object
    ///
    /// The starting point of the associated Publish.
    ///
    /// Some if `filter_type` is:
    ///
    /// * [AbsoluteStart](FilterType::AbsoluteStart)
    /// * [AbsoluteRange](FilterType::AbsoluteRange)
    ///
    /// Otherwise None.
    ///
    /// [Location]
    #[varint(when(filter_type = 0x3 || 0x4))]
    pub start_location: x!([Location]),

    /// ## Final Group
    ///
    /// The final Group of this Track.
    ///
    /// Some if `filter_type` is:
    ///
    /// * [AbsoluteRange](FilterType::AbsoluteRange)
    ///
    /// Otherwise None.
    #[varint(when(filter_type = 0x4))]
    pub end_group: x!([i]),

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for PublishOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                request_id: 9u8.into(),
                forward: Forward::Disabled,
                subscriber_priority: 35.try_into().expect("will fit"),
                group_order: GroupOrder::Original,
                filter_type: FilterType::AbsoluteRange,
                start_location: Some((3u8, 1u8).into()),
                end_group: Some(50u8.into()),
                parameters: Default::default(),
            };
            let b1 = vec![
                9,  // ID 9
                0,  // forward disabled
                35, // sub prio 80
                0,  // original group order
                4,  // absolute range filter type
                3,  // start group 3
                1,  // start object 1
                50, // end group 50
                0,  // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self {
                request_id: 10u8.into(),
                forward: Forward::Enabled,
                subscriber_priority: 5.try_into().expect("will fit"),
                group_order: GroupOrder::Ascending,
                filter_type: FilterType::AbsoluteStart,
                start_location: Some((1u8, 1u8).into()),
                end_group: None,
                parameters: Default::default(),
            };
            let b2 = vec![
                10, // ID
                1,  // forward enabled
                5,  // sub prio
                1,  // ascending group order
                3,  // absolute start
                1,  // start group
                1,  // start object
                // end group not needed
                0, // no parameters
            ];
            let l2 = b2.len() * 8;

            let v3 = Self {
                request_id: 10u8.into(),
                forward: Forward::Enabled,
                subscriber_priority: 5.try_into().expect("will fit"),
                group_order: GroupOrder::Ascending,
                filter_type: FilterType::NextGroupStart,
                start_location: None,
                end_group: None,
                parameters: Default::default(),
            };
            let b3 = vec![
                10, // ID
                1,  // forward enabled
                5,  // sub prio
                1,  // ascending group order
                1,  // next group start
                // start not needed
                // end group not needed
                0, // no parameters
            ];
            let l3 = b3.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3)]
        }
    }

    varint_struct_test!(PublishOk);
}
