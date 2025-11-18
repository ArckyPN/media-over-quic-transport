mod error;
mod ok;
mod un;
mod update;

pub use {error::SubscribeError, ok::SubscribeOk, un::Unsubscribe, update::SubscribeUpdate};

use {
    crate::types::{
        Parameters,
        misc::{FilterType, Forward, GroupOrder, Location},
        track,
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
#[varint(parameters(auth_token, delivery_timeout))]
pub struct Subscribe {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Track Namespace
    ///
    /// The Namespace of the Track.
    ///
    /// [Namespace](track::Namespace)
    pub track_namespace: track::Namespace,

    /// ## Track Name
    ///
    /// The Name of the Track.
    ///
    /// [Name](track::Name)
    pub track_name: track::Name,

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

impl<S: subscribe_builder::State> SubscribeBuilder<S> {
    // param_builder! {}
}

#[bon]
impl Subscribe {
    #[builder]
    pub fn new(
        #[builder(field)] parameters: Parameters,
        #[builder(into)] request_id: x!(i),
        #[builder(into)] track_namespace: track::Namespace,
        #[builder(into)] track_name: track::Name,
        #[builder(
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// TODO docs
                }
        ))]
        subscriber_priority: x!(8),
        #[builder(into)] group_order: GroupOrder,
        #[builder(into)] forward: Forward,
        #[builder(into)] filter_type: FilterType,
        #[builder(into)] start_location: x!([Location]),
        #[builder(into)] end_group: x!([i]),
    ) -> Self {
        Self {
            request_id,
            track_namespace,
            track_name,
            subscriber_priority,
            group_order,
            forward,
            filter_type,
            start_location,
            end_group,
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
            // let v1 = Self::builder()
            //     .request_id(15u8)
            //     .track_namespace(["num", "boom"])
            //     .track_name("bob")
            //     .subscriber_priority(50.try_into().expect("will fit")) // TODO remaining params
            //     .build();
            let v1 = Self {
                request_id: 15u8.into(),
                track_namespace: ["num", "boom"].into(),
                track_name: "bob".into(),
                subscriber_priority: 50.try_into().expect("will fit"),
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
