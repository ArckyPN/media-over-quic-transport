use varint::{VarInt, x};

use crate::types::{
    Parameters,
    misc::{EndOfTrack, GroupOrder, Location},
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

impl FetchOk {
    // TODO bon builder
    pub fn new<ID, G, E, L>(
        id: ID,
        group_order: G,
        end_of_track: E,
        end_location: L,
        params: Option<Parameters>,
    ) -> Self
    where
        ID: Into<x!(i)>,
        G: Into<GroupOrder>,
        E: Into<EndOfTrack>,
        L: Into<Location>,
    {
        Self {
            request_id: id.into(),
            group_order: group_order.into(),
            end_of_track: end_of_track.into(),
            end_location: end_location.into(),
            parameters: params.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for FetchOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(
                4u8,
                GroupOrder::Original,
                EndOfTrack::True,
                (54u8, 3u8),
                None,
            );
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
