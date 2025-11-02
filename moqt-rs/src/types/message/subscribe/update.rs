use varint::{VarInt, x};

use crate::types::{Parameters, misc::Location};

/// ## SubscribeUpdate
///
/// Modify an active [Subscribe](crate::type::message::Subscribe).
///
/// A Subscription can only be narrowed and not widened, i.e.
/// `start_location` must not decrease and `end_group`
/// must not increase.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token, delivery_timeout))]
pub struct SubscribeUpdate {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## First Object
    ///
    /// The new starting Object.
    ///
    /// [Location]
    pub start_location: Location,

    /// ## Final Group
    ///
    /// The new final Group.
    pub end_group: x!(i),

    /// ## Subscriber Priority
    ///
    /// Sets a priority in relation to all Fetches
    /// and Subscribes in the current Session.
    ///
    /// Lower means higher priority.
    pub subscriber_priority: x!(8),

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
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
