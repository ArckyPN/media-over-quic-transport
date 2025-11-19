mod error;
mod ok;
mod un;
mod update;

pub use {error::SubscribeError, ok::SubscribeOk, un::Unsubscribe, update::SubscribeUpdate};

use {
    crate::types::{
        Parameters,
        misc::{FilterType, Forward, GroupOrder, Location},
        track::{Name, Namespace},
    },
    bon::bon,
    varint::{VarInt, x},
};

/// ## Subscribe
///
/// Request Objects from an ongoing Track.
///
/// There are four types of Subscribe,
/// indicated by the [FilterType]:
///
/// ### NextGroupStart
///
/// The first Object received will be the
/// first Object of the next published
/// Group.
///
/// This Subscription is open-ended.
///
/// ### LargestObject
///
/// The first Object received will be the
/// next published Object of the current
/// Group.
///
/// This Subscription is open-ended.
///
/// ### AbsoluteStart
///
/// The first Object received will be the one
/// specified by the Subscribe message.
///
/// This Subscription is open-ended.
///
/// ### AbsoluteRange
///
/// The Subscription will be active for the
/// specified range of Objects.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(
    parameters(auth_token, delivery_timeout),
    builder = with_next_group_start,
    builder = with_largest_object,
    builder = with_absolute_start,
    builder = with_absolute_range,
)]
pub struct Subscribe {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Track Namespace
    ///
    /// The Namespace of the Track.
    ///
    /// [Namespace]
    pub namespace: Namespace,

    /// ## Track Name
    ///
    /// The Name of the Track.
    ///
    /// [Name]
    pub name: Name,

    /// ## Subscriber Priority
    ///
    /// Sets a priority in relation to all Fetches
    /// and Subscribes in the current Session.
    ///
    /// Lower means higher priority.
    pub subscriber_priority: x!(8),

    /// ## Group Order
    ///
    /// Indicates the requested order of Group.
    pub group_order: GroupOrder,

    /// ## Forward Mode
    ///
    /// Indicates the forwarding mode.
    ///
    /// - [Enabled](Forward::Enabled):
    /// Objects will be transmitted
    /// - [Disabled](Forward::Disabled):
    /// Object will not be transmitted
    ///
    /// [Forward]
    pub forward: Forward,

    /// ## Filter Type
    ///
    /// Indicates the Subscribe mode.
    ///
    /// [FilterType]
    pub filter_type: FilterType,

    /// ## First Object
    ///
    /// Specifies the first Object.
    ///
    /// Some when `filter_type` is:
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
    /// Specifies the final Group.
    ///
    /// Some when `filter_type` is:
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

#[bon]
impl Subscribe {
    /// Creates a builder for [Subscribe] with [FilterType::NextGroupStart].
    #[builder(finish_fn = build)]
    pub fn with_next_group_start(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [Subscribe].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the track namespace on [Subscribe].
            }
        ))]
        namespace: Namespace,

        #[builder(into, setters(
            doc {
                /// Sets the track name on [Subscribe].
            }
        ))]
        name: Name,

        #[builder(
            name = sub_prio,
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// Sets the subscriber priority on [Subscribe].
                }
        ))]
        subscriber_priority: x!(8),

        #[builder(setters(
            doc {
                /// Sets the group order on [Subscribe].
            }
        ))]
        group_order: GroupOrder,

        #[builder(into, setters(
            doc {
                /// Sets the forwarding on [Subscribe].
            }
        ))]
        forward: Forward,
    ) -> Self {
        Self {
            request_id,
            namespace,
            name,
            subscriber_priority,
            group_order,
            forward,
            filter_type: FilterType::NextGroupStart,
            start_location: None,
            end_group: None,
            parameters,
        }
    }

    /// Creates a builder for [Subscribe] with [FilterType::LargestObject].
    #[builder(finish_fn = build)]
    pub fn with_largest_object(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [Subscribe].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the track namespace on [Subscribe].
            }
        ))]
        namespace: Namespace,

        #[builder(into, setters(
            doc {
                /// Sets the track name on [Subscribe].
            }
        ))]
        name: Name,

        #[builder(
            name = sub_prio,
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// Sets the subscriber priority on [Subscribe].
                }
        ))]
        subscriber_priority: x!(8),

        #[builder(setters(
            doc {
                /// Sets the group order on [Subscribe].
            }
        ))]
        group_order: GroupOrder,

        #[builder(into, setters(
            doc {
                /// Sets the forwarding on [Subscribe].
            }
        ))]
        forward: Forward,
    ) -> Self {
        Self {
            request_id,
            namespace,
            name,
            subscriber_priority,
            group_order,
            forward,
            filter_type: FilterType::LargestObject,
            start_location: None,
            end_group: None,
            parameters,
        }
    }

    /// Creates a builder for [Subscribe] with [FilterType::AbsoluteStart].
    #[builder(finish_fn = build)]
    pub fn with_absolute_start(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [Subscribe].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the track namespace on [Subscribe].
            }
        ))]
        namespace: Namespace,

        #[builder(into, setters(
            doc {
                /// Sets the track name on [Subscribe].
            }
        ))]
        name: Name,

        #[builder(
            name = sub_prio,
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// Sets the subscriber priority on [Subscribe].
                }
        ))]
        subscriber_priority: x!(8),

        #[builder(setters(
            doc {
                /// Sets the group order on [Subscribe].
            }
        ))]
        group_order: GroupOrder,

        #[builder(into, setters(
            doc {
                /// Sets the forwarding on [Subscribe].
            }
        ))]
        forward: Forward,

        #[builder(
            with = |group: impl Into<varint::x!(i)>, object: impl Into<varint::x!(i)>| (group.into(), object.into()).into(),
            setters(
            name = start,
            doc {
                /// Sets the start location on [Subscribe].
            }
        ))]
        start_location: Location,
    ) -> Self {
        Self {
            request_id,
            namespace,
            name,
            subscriber_priority,
            group_order,
            forward,
            filter_type: FilterType::AbsoluteStart,
            start_location: Some(start_location),
            end_group: None,
            parameters,
        }
    }

    /// Creates a builder for [Subscribe] with [FilterType::AbsoluteRange].
    #[builder(finish_fn = build)]
    pub fn with_absolute_range(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [Subscribe].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the track namespace on [Subscribe].
            }
        ))]
        namespace: Namespace,

        #[builder(into, setters(
            doc {
                /// Sets the track name on [Subscribe].
            }
        ))]
        name: Name,

        #[builder(
            name = sub_prio,
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// Sets the subscriber priority on [Subscribe].
                }
        ))]
        subscriber_priority: x!(8),

        #[builder(setters(
            doc {
                /// Sets the group order on [Subscribe].
            }
        ))]
        group_order: GroupOrder,

        #[builder(into, setters(
            doc {
                /// Sets the forwarding on [Subscribe].
            }
        ))]
        forward: Forward,

        #[builder(
            with = |group: impl Into<varint::x!(i)>, object: impl Into<varint::x!(i)>| (group.into(), object.into()).into(),
            setters(
            name = start,
            doc {
                /// Sets the start location on [Subscribe].
            }
        ))]
        start_location: Location,

        #[builder(into, setters(
            doc {
                /// Sets the end group on [Subscribe].
            }
        ))]
        end_group: x!(i),
    ) -> Self {
        Self {
            request_id,
            namespace,
            name,
            subscriber_priority,
            group_order,
            forward,
            filter_type: FilterType::AbsoluteRange,
            start_location: Some(start_location),
            end_group: Some(end_group),
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Subscribe {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::with_absolute_start()
                .id(15u8)
                .namespace(["num", "boom"])
                .name("bob")
                .sub_prio(50)
                .group_order(GroupOrder::Original)
                .forward(true)
                .start(5u8, 1u8)
                .build();
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
