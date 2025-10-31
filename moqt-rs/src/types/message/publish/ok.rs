use varint::{VarInt, x};

use crate::types::{
    location::Location,
    misc::{FilterType, Forward, GroupOrder},
};

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
pub struct PublishOk {
    /// TODO docs
    request_id: x!(i),
    /// TODO docs
    forward: Forward,
    /// TODO docs
    subscriber_priority: x!(8),
    /// TODO docs
    group_order: GroupOrder,
    /// TODO docs
    filter_type: FilterType,
    /// TODO docs
    #[varint(when(filter_type = 0x3 || 0x4))]
    start_location: x!([Location]),
    /// TODO docs
    #[varint(when(filter_type = 0x4))]
    end_group: x!([i]),
    // TODO parameters
}
// TODO impls for usability

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for PublishOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                request_id: 9u8.into(),
                forward: Forward::Disabled,
                subscriber_priority: 35.try_into().unwrap(),
                group_order: GroupOrder::Original,
                filter_type: FilterType::AbsoluteRange,
                start_location: Some((3u8, 1u8).into()),
                end_group: Some(50u8.into()),
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
            ];
            let l1 = b1.len() * 8;

            let v2 = Self {
                request_id: 10u8.into(),
                forward: Forward::Enabled,
                subscriber_priority: 5.try_into().unwrap(),
                group_order: GroupOrder::Ascending,
                filter_type: FilterType::AbsoluteStart,
                start_location: Some((1u8, 1u8).into()),
                end_group: None,
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
            ];
            let l2 = b2.len() * 8;

            let v3 = Self {
                request_id: 10u8.into(),
                forward: Forward::Enabled,
                subscriber_priority: 5.try_into().unwrap(),
                group_order: GroupOrder::Ascending,
                filter_type: FilterType::NextGroupStart,
                start_location: None,
                end_group: None,
            };
            let b3 = vec![
                10, // ID
                1,  // forward enabled
                5,  // sub prio
                1,  // ascending group order
                1,  // next group start
                    // start not needed
                    // end group not needed
            ];
            let l3 = b3.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3)]
        }
    }

    varint_struct_test!(PublishOk);
}
