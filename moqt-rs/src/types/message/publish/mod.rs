mod done;
mod error;
mod ok;

pub use {done::PublishDone, error::PublishError, ok::PublishOk};

use {
    crate::types::{
        Parameters,
        misc::{ContentExists, Forward, GroupOrder, Location},
        track::{Name, Namespace},
    },
    bon::bon,
    varint::{VarInt, x},
};

/// ## Publish
///
/// Initiates the publishing of a new Track.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token, max_cache_duration))]
pub struct Publish {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Track Namespace
    ///
    /// The Namespace of the new Track.
    ///
    /// [Namespace]
    pub namespace: Namespace,

    /// ## Track Name
    ///
    /// The Name of the new Track.
    ///
    /// [Name](track::Name)
    pub name: Name,

    /// ## Track Alias
    ///
    /// The Track Alias of this Track.
    pub alias: x!(i), // TODO new type track::Alias?

    /// The Order in which Groups will be published.
    ///
    /// [GroupOrder]
    pub group_order: GroupOrder,

    /// ## Content Exists
    ///
    /// Indicates whether an Object has already
    /// been published.
    ///
    /// [ContentExists]
    pub content_exists: ContentExists,

    /// ## Final Object
    ///
    /// The Group an Object ID of the largest
    /// Object available for this Track.
    ///
    /// Some when `content_exists` is:
    ///
    /// * [Yes](ContentExists::Yes)
    ///
    /// Otherwise None.
    ///
    /// [Location]
    #[varint(when(content_exists = 0x1))]
    pub largest_location: x!([Location]),

    /// ## Forward Mode
    ///
    /// Sets the mode of forwarding Objects.
    ///
    /// [Forward]
    pub forward: Forward,

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

use publish_builder::{IsUnset, SetContentExists, SetLargestLocation, State};
impl<S: State> PublishBuilder<S>
where
    S::ContentExists: IsUnset,
    S::LargestLocation: IsUnset,
{
    pub fn with_content<G, O>(
        self,
        group: G,
        object: O,
    ) -> PublishBuilder<SetContentExists<SetLargestLocation<S>>>
    where
        G: Into<x!(i)>,
        O: Into<x!(i)>,
    {
        let this = self.largest_location_internal(Some((group.into(), object.into()).into()));
        this.content_exists_internal(ContentExists::Yes)
    }
}

#[bon]
impl Publish {
    #[builder(finish_fn = build)]
    pub fn new(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [Publish].
            }
        ))]
        request_id: x!(i),

        #[builder(into, setters(
            doc {
                /// Sets the track namespace on [Publish].
            }
        ))]
        namespace: Namespace,

        #[builder(into, setters(
            doc {
                /// Sets the track name on [Publish].
            }
        ))]
        name: Name,

        #[builder(into, setters(
            doc {
                /// Sets the session alias on [Publish].
            }
        ))]
        alias: x!(i),

        #[builder(setters(
            doc {
                /// Sets the group order on [Publish].
            }
        ))]
        group_order: GroupOrder,

        #[builder(default, setters(vis = "", name = content_exists_internal))]
        content_exists: ContentExists,

        #[builder(default, setters(vis = "", name = largest_location_internal))] largest_location: x!(
            [Location]
        ),

        #[builder(into, setters(
            doc {
                /// Sets the forwarding on [Publish].
            }
        ))]
        forward: Forward,
    ) -> Self {
        Self {
            request_id,
            namespace,
            name,
            alias,
            group_order,
            content_exists,
            largest_location,
            forward,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Publish {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .id(9u8)
                .namespace(["moq"])
                .name("vod")
                .alias(5u8)
                .group_order(GroupOrder::Original)
                .forward(true)
                .build();
            let b1 = vec![
                9, // ID 9
                1, // 1 tuple namespace
                3, // len of namespace
                b'm', b'o', b'q', // namespace "moq"
                3,    // len of name
                b'v', b'o', b'd', // name "vod"
                5,    // alias 5
                0,    // original group order
                0,    // content doesn't exist
                // larger loc not needed
                1, // forward enabled
                0, // no parameters
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::builder()
                .id(9u8)
                .namespace(["moq"])
                .name("vod")
                .alias(5u8)
                .group_order(GroupOrder::Original)
                .with_content(43u8, 15u8)
                .forward(true)
                .build();
            let b2 = vec![
                9, // ID 9
                1, // 1 tuple namespace
                3, // len of namespace
                b'm', b'o', b'q', // namespace "moq"
                3,    // len of name
                b'v', b'o', b'd', // name "vod"
                5,    // alias 5
                0,    // original group order
                1,    // content exist
                43,   // largest group
                15,   // largest object
                1,    // forward enabled
                0,    // no parameters
            ];
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(Publish);
}
