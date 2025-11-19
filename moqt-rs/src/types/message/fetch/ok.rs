use {
    crate::types::{
        Parameters,
        misc::{EndOfTrack, GroupOrder, Location},
    },
    bon::bon,
    varint::{VarInt, x},
};

/// ## FetchOk
///
/// Response to a successful [Fetch](crate::types::message::Fetch)
/// Message.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(max_cache_duration))]
pub struct FetchOk {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Group Order
    ///
    /// The order in which Groups will be
    /// sent.
    ///
    /// [GroupOrder]
    pub group_order: GroupOrder,

    /// ## End of Track
    ///
    /// Indicates whether this Track has
    /// ended or is still receiving new
    /// Objects.
    ///
    /// [EndOfTrack]
    pub end_of_track: EndOfTrack,

    /// ## Final Object
    ///
    /// The largest Objects covered by this
    /// Fetch.
    ///
    /// [Location]
    pub end_location: Location,

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

#[bon]
impl FetchOk {
    #[builder]
    pub fn new(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [FetchOk].
            }
        ))]
        request_id: x!(i),

        #[builder(setters(
            doc {
                /// Sets the group order on [FetchOk].
            }
        ))]
        group_order: GroupOrder,

        #[builder(into, setters(
            doc {
                /// Sets end of track on [FetchOk].
            }
        ))]
        end_of_track: EndOfTrack,

        #[builder(
            with = |group: impl Into<varint::x!(i)>, object: impl Into<varint::x!(i)>| (group.into(), object.into()).into(),
            setters(
            doc {
                /// Sets the end location on [FetchOk].
            }
        ))]
        end_location: Location,
    ) -> Self {
        Self {
            request_id,
            group_order,
            end_of_track,
            end_location,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for FetchOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .id(4u8)
                .group_order(GroupOrder::Original)
                .end_of_track(true)
                .end_location(54u8, 3u8)
                .build();
            let b1 = vec![
                4,  // ID
                0,  // original group order
                1,  // is end of track
                54, // end group
                3,  //end object
                0,  // no parameters
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(FetchOk);
}
