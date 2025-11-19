use {
    crate::types::{
        Parameters,
        misc::{FilterType, Forward, GroupOrder, Location},
    },
    bon::bon,
    varint::{VarInt, x},
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

use publish_ok_builder::{IsUnset, SetEndGroup, SetFilterType, SetStartLocation, State};
impl<S: State> PublishOkBuilder<S>
where
    S::FilterType: IsUnset,
    S::StartLocation: IsUnset,
    S::EndGroup: IsUnset,
{
    /// Set the [FilterType] to NextGroupStart.
    pub fn with_next_group_start(
        self,
    ) -> PublishOkBuilder<SetFilterType<SetStartLocation<SetEndGroup<S>>>> {
        let this = self.end_group_internal(None);
        let this = this.start_location_internal(None);
        this.filter_type_internal(FilterType::NextGroupStart)
    }

    /// Set the [FilterType] to LargestObject.
    pub fn with_largest_object(
        self,
    ) -> PublishOkBuilder<SetFilterType<SetStartLocation<SetEndGroup<S>>>> {
        let this = self.end_group_internal(None);
        let this = this.start_location_internal(None);
        this.filter_type_internal(FilterType::LargestObject)
    }

    pub fn with_absolute_start<G, O>(
        self,
        group: G,
        object: O,
    ) -> PublishOkBuilder<SetFilterType<SetStartLocation<SetEndGroup<S>>>>
    where
        G: Into<x!(i)>,
        O: Into<x!(i)>,
    {
        let this = self.end_group_internal(None);
        let this = this.start_location_internal(Some((group.into(), object.into()).into()));
        this.filter_type_internal(FilterType::AbsoluteStart)
    }

    pub fn with_absolute_range<L, E>(
        self,
        start: L,
        end_group: E,
    ) -> PublishOkBuilder<SetFilterType<SetStartLocation<SetEndGroup<S>>>>
    where
        L: Into<Location>,
        E: Into<x!(i)>,
    {
        let this = self.end_group_internal(Some(end_group.into()));
        let this = this.start_location_internal(Some(start.into()));
        this.filter_type_internal(FilterType::AbsoluteRange)
    }
}

#[bon]
impl PublishOk {
    #[builder(finish_fn = build)]
    pub fn new(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [PublishOk].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the forwarding on [PublishOk].
            }
        ))]
        forward: Forward,

        #[builder(
            name = sub_prio,
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// Sets the subscriber priority on [PublishOk].
                }
        ))]
        subscriber_priority: x!(8),

        #[builder(setters (
            doc {
                /// Sets the group order on [PublishOk].
            }
        ))]
        group_order: GroupOrder,

        #[builder(setters(vis = "", name = filter_type_internal))] filter_type: FilterType,

        #[builder(setters(vis = "", name = start_location_internal))] start_location: x!([
            Location
        ]),

        #[builder(setters(vis = "", name = end_group_internal))] end_group: x!([i]),
    ) -> Self {
        Self {
            request_id,
            forward,
            subscriber_priority,
            group_order,
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

    impl TestData for PublishOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .id(9u8)
                .forward(false)
                .sub_prio(35)
                .group_order(GroupOrder::Original)
                .with_absolute_range((3u8, 1u8), 50u8)
                .build();
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

            let v2 = Self::builder()
                .id(10u8)
                .forward(true)
                .sub_prio(5)
                .group_order(GroupOrder::Ascending)
                .with_absolute_start(1u8, 1u8)
                .build();
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

            let v3 = Self::builder()
                .id(10u8)
                .forward(true)
                .sub_prio(5)
                .group_order(GroupOrder::Ascending)
                .with_next_group_start()
                .build();
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
