// pub use {error::SubscribeError, ok::SubscribeOk, un::Unsubscribe, update::SubscribeUpdate};

use crate::types::config::DEFAULT_PRIORITY;

use {
    crate::types::{
        Parameters,
        misc::{FilterType, Forward, GroupOrder, Location},
    },
    bon::bon,
    varint::{VarInt, x},
};

/// ## [Subscribe](crate::types::message::Subscribe)Config
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint(parameters(auth_token, delivery_timeout))]
//
pub struct SubscribeConfig {
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
    ///   Objects will be transmitted
    /// - [Disabled](Forward::Disabled):
    ///   Object will not be transmitted
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

use subscribe_config_builder::{IsUnset, SetEndGroup, SetFilterType, SetStartLocation, State};
impl<S: State> SubscribeConfigBuilder<S>
where
    S::FilterType: IsUnset,
    S::StartLocation: IsUnset,
    S::EndGroup: IsUnset,
{
    pub fn with_next_group_start(
        self,
    ) -> SubscribeConfigBuilder<SetFilterType<SetEndGroup<SetStartLocation<S>>>> {
        let this = self.start_location_internal(None);
        let this = this.end_group_internal(None);
        this.filter_type_internal(FilterType::NextGroupStart)
    }

    pub fn with_largest_object(
        self,
    ) -> SubscribeConfigBuilder<SetFilterType<SetEndGroup<SetStartLocation<S>>>> {
        let this = self.start_location_internal(None);
        let this = this.end_group_internal(None);
        this.filter_type_internal(FilterType::LargestObject)
    }

    pub fn with_absolute_start<G, O>(
        self,
        group: G,
        object: O,
    ) -> SubscribeConfigBuilder<SetFilterType<SetEndGroup<SetStartLocation<S>>>>
    where
        G: Into<x!(i)>,
        O: Into<x!(i)>,
    {
        let this = self.start_location_internal(Some((group.into(), object.into()).into()));
        let this = this.end_group_internal(None);
        this.filter_type_internal(FilterType::AbsoluteStart)
    }

    pub fn with_absolute_range<L, E>(
        self,
        start: L,
        end_group: E,
    ) -> SubscribeConfigBuilder<SetFilterType<SetEndGroup<SetStartLocation<S>>>>
    where
        L: Into<Location>,
        E: Into<x!(i)>,
    {
        let this = self.start_location_internal(Some(start.into()));
        let this = this.end_group_internal(Some(end_group.into()));
        this.filter_type_internal(FilterType::AbsoluteStart)
    }
}

#[bon]
impl SubscribeConfig {
    #[builder]
    pub fn new(
        #[builder(field)] parameters: Parameters,
        #[builder(setters(vis = "", name = filter_type_internal))] filter_type: FilterType,
        #[builder(setters(vis = "", name = start_location_internal))] start_location: x!([
            Location
        ]),
        #[builder(setters(vis = "", name = end_group_internal))] end_group: x!([i]),

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

impl Default for SubscribeConfig {
    fn default() -> Self {
        Self::builder()
            .sub_prio(DEFAULT_PRIORITY)
            .group_order(GroupOrder::Original)
            .forward(Forward::Disabled)
            .with_next_group_start()
            .build()
    }
}
