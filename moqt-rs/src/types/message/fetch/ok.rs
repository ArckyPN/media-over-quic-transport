use varint::{VarInt, x};

use crate::types::{location::Location, misc::EndOfTrack, misc::GroupOrder};

#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct FetchOk {
    request_id: x!(i),
    group_order: GroupOrder,
    end_of_track: EndOfTrack,
    end_location: Location,
    // TODO parameters
}
// TODO impls for usability

impl FetchOk {
    pub fn new<ID, G, E, L>(id: ID, group_order: G, end_of_track: E, end_location: L) -> Self
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for FetchOk {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(4u8, GroupOrder::Original, EndOfTrack::True, (54u8, 3u8));
            let b1 = vec![
                4,  // ID
                0,  // original group order
                1,  // is end of track
                54, // end group
                3,  //end object
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(FetchOk);
}
