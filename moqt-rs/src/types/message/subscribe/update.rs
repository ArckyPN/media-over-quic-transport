use varint::{VarInt, x};

use crate::types::{Parameters, location::Location};

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token, delivery_timeout))]
pub struct SubscribeUpdate {
    /// TODO docs
    request_id: x!(i),
    /// TODO docs
    start_location: Location,
    /// TODO docs
    end_group: x!(i),
    /// TODO docs
    subscriber_priority: x!(8),
    // TODO doc
    parameters: Parameters,
}

impl SubscribeUpdate {
    // pub fn new<ID, S, E, P>(id: ID, start: S, end: E, prio: P) -> Self
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for SubscribeUpdate {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                request_id: 9u8.into(),
                start_location: (13u8, 1u8).into(),
                end_group: 50u8.into(),
                subscriber_priority: 0.try_into().unwrap(),
                parameters: Default::default(),
            };
            let b1 = vec![
                9,  // ID 9
                13, // start group 13
                1,  // start object 1
                50, // end group 50
                0,  // sub prio
                0,  // no parameters
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(SubscribeUpdate);
}
