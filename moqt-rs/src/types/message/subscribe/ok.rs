use {
    crate::types::{
        Parameters,
        misc::{ContentExists, GroupOrder, Location},
    },
    bon::bon,
    std::time::Duration,
    varint::{VarInt, x},
};

/// ## SubscribeOk
///
/// Response to a successful [Subscribe](crate::type::message::Subscribe).
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(delivery_timeout, max_cache_duration))]
pub struct SubscribeOk {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Track Alias
    ///
    /// The assigned Track Alias.
    pub alias: x!(i),

    /// ## Expiry
    ///
    /// Number of Milliseconds after which
    /// the Subscription will expire.
    ///
    /// 0 indicates no expiration.
    pub expires: Duration,

    /// ## Group Order
    ///
    /// The send order of Groups.
    ///
    /// [GroupOrder]
    pub group_order: GroupOrder,

    /// ## Content Exists
    ///
    /// Whether or not Objects have already
    /// been published on this Track.
    ///
    /// [ContentExists]
    pub content_exists: ContentExists,

    /// ## Final Object
    ///
    /// The largest Object, if any have been
    /// published. As indicated by `content_exists`.
    ///
    /// [Location]
    #[varint(when(content_exists = 0x1))]
    pub largest_location: x!([Location]),

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

use subscribe_ok_builder::{IsUnset, SetContentExists, SetLargestLocation, State};
impl<S: State> SubscribeOkBuilder<S>
where
    S::ContentExists: IsUnset,
    S::LargestLocation: IsUnset,
{
    /// Optional setter for [`content_exists`](SubscribeOk::content_exists) and
    /// [`largest_location`](SubscribeOk::largest_location) on [SubscribeOk].
    pub fn with_content<G, O>(
        self,
        group: G,
        object: O,
    ) -> SubscribeOkBuilder<SetContentExists<SetLargestLocation<S>>>
    where
        G: Into<x!(i)>,
        O: Into<x!(i)>,
    {
        let this = self.largest_location_internal(Some((group.into(), object.into()).into()));
        this.content_exists_internal(ContentExists::Yes)
    }
}

#[bon]
impl SubscribeOk {
    /// Creates a Builder for [SubscribeOk].
    #[builder]
    pub fn new(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [SubscribeOk].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the track alias on [SubscribeOk].
            }
        ))]
        alias: x!(i),

        #[builder(setters(
            doc {
                /// Sets the expiration duration on [SubscribeOk].
            }
        ))]
        expires: Duration,

        #[builder(setters(
            doc {
                /// Sets the group order on [SubscribeOk].
            }
        ))]
        group_order: GroupOrder,

        #[builder(default, setters(vis = "", name = content_exists_internal))]
        content_exists: ContentExists,
        #[builder(default, setters(vis = "", name = largest_location_internal))] largest_location: x!(
            [Location]
        ),
    ) -> Self {
        Self {
            request_id,
            alias,
            expires,
            group_order,
            content_exists,
            largest_location,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for SubscribeOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .id(9u8)
                .alias(13u8)
                .expires(Duration::from_millis(10))
                .group_order(GroupOrder::Original)
                .build();
            let b1 = vec![
                9,  // ID 9
                13, // track alias 13
                10, // expires in 10ms
                0,  // original group order
                0,  // content doesn't exist
                // largest loc not needed
                0, // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::builder()
                .id(1u8)
                .alias(3u8)
                .expires(Duration::from_millis(15))
                .group_order(GroupOrder::Ascending)
                .with_content(5u8, 5u8)
                .build();
            let b2 = vec![
                1,  // ID
                3,  // alias
                15, // expires
                1,  // ascending group order
                1,  // content exists
                5,  // largest group
                5,  // largest object
                0,  // no parameters
            ];
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(SubscribeOk);
}
