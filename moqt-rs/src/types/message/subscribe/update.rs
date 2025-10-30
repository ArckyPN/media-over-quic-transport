use varint::{VarInt, x};

use crate::types::location::Location;

#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct SubscribeUpdate {
    request_id: x!(i),
    start_location: Location,
    end_group: x!(i),
    subscriber_priority: x!(8),
    // TODO parameters
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
            };
            let b1 = vec![
                9,  // ID 9
                13, // start group 13
                1,  // start object 1
                50, // end group 50
                0,  // sub prio
                    // TODO parameters
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(SubscribeUpdate);
}
