mod error;
mod ok;
mod un;
mod update;

pub use error::SubscribeError;
pub use ok::SubscribeOk;
pub use un::Unsubscribe;
pub use update::SubscribeUpdate;

use varint::{VarInt, x};

use crate::types::{
    Parameters,
    location::Location,
    misc::{ContentExists, FilterType, Forward, GroupOrder},
    track,
};

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token, delivery_timeout))]
pub struct Subscribe {
    /// TODO docs
    request_id: x!(i),
    /// TODO docs
    track_namespace: track::Namespace,
    /// TODO docs
    track_name: track::Name,
    /// TODO docs
    subscriber_priority: x!(8),
    /// TODO docs
    group_order: GroupOrder,
    /// TODO docs
    forward: Forward,
    /// TODO docs
    filter_type: FilterType,
    /// TODO docs
    #[varint(when(filter_type = 0x3 || 0x4))]
    start_location: x!([Location]),
    /// TODO docs
    #[varint(when(filter_type = 0x4))]
    end_group: x!([i]),
    // TODO doc
    parameters: Parameters,
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Subscribe {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                request_id: 15u8.into(),
                track_namespace: ["num", "boom"].into(),
                track_name: "bob".into(),
                subscriber_priority: 50.try_into().unwrap(),
                group_order: GroupOrder::Original,
                forward: Forward::Enabled,
                filter_type: FilterType::AbsoluteStart,
                start_location: Some((5u8, 1u8).into()),
                end_group: None,
                parameters: Default::default(),
            };
            let b1 = [
                vec![
                    15, // request id: 15
                    2,  // 2 element tuple
                    3,  // first tuple len 3
                ],
                b"num".to_vec(),
                vec![
                    4, // second tuple len 4
                ],
                b"boom".to_vec(),
                vec![
                    3, // track name len 3
                ],
                b"bob".to_vec(),
                vec![
                    50, // subscriber priority 50
                    0,  // original group order
                    1,  // enable forward
                    3,  // filter type absolute start
                    5,  // start group 5 (location)
                    1,  // start object 1 (location)
                    // end group not needed because filter 3
                    0, // no parameters
                ],
            ]
            .concat();
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(Subscribe);
}
