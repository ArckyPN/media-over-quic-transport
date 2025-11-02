use std::time::Duration;

use varint::{VarInt, x};

use crate::types::{
    Parameters,
    misc::{ContentExists, GroupOrder, Location},
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
    pub track_alias: x!(i),

    /// ## Expiry
    ///
    /// Number of Milliseconds after which
    /// the Subscription will expire.
    ///
    /// 0 indicates no expiration.
    pub expires: x!(i),

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

impl SubscribeOk {
    pub fn new_with_content<ID, A, E, G, L>(
        id: ID,
        alias: A,
        expires: E,
        group: G,
        location: Option<L>,
    ) -> Self
    where
        ID: Into<x!(i)>,
        A: Into<x!(i)>,
        E: Into<x!(i)>,
        G: Into<GroupOrder>,
        L: Into<Location>,
    {
        Self {
            request_id: id.into(),
            track_alias: alias.into(),
            expires: expires.into(),
            group_order: group.into(),
            content_exists: ContentExists::Yes,
            largest_location: location.map(Into::into),
            parameters: Default::default(),
        }
    }

    pub fn new_no_content<ID, A, E, G>(id: ID, alias: A, expires: E, group: G) -> Self
    where
        ID: Into<x!(i)>,
        A: Into<x!(i)>,
        E: Into<x!(i)>,
        G: Into<GroupOrder>,
    {
        Self {
            request_id: id.into(),
            track_alias: alias.into(),
            expires: expires.into(),
            group_order: group.into(),
            content_exists: ContentExists::No,
            largest_location: None,
            parameters: Default::default(),
        }
    }

    pub fn expires(&self) -> Duration {
        Duration::from_millis(self.expires.number())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for SubscribeOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new_no_content(9u8, 13u8, 10u8, GroupOrder::Original);
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

            let v2 =
                Self::new_with_content(1u8, 3u8, 15u8, GroupOrder::Ascending, Some((5u8, 5u8)));
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
