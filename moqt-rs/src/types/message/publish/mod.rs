mod done;
mod error;
mod ok;

pub use done::PublishDone;
pub use error::PublishError;
pub use ok::PublishOk;

use varint::{VarInt, x};

use crate::types::{
    location::Location,
    misc::{ContentExists, Forward, GroupOrder},
    track,
};

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
pub struct Publish {
    /// TODO docs
    request_id: x!(i),
    /// TODO docs
    namespace: track::Namespace,
    /// TODO docs
    name: track::Name,
    /// TODO docs
    alias: x!(i), // TODO new type track::Alias?
    /// TODO docs
    group_order: GroupOrder,
    /// TODO docs
    content_exists: ContentExists,
    /// TODO docs
    #[varint(when(content_exists = 0x1))]
    largest_location: x!([Location]),
    /// TODO docs
    forward: Forward,
    // TODO parameters
}
// TODO impls for usability

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Publish {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self {
                request_id: 9u8.into(),
                namespace: ["moq"].into(),
                name: "vod".into(),
                alias: 5u8.into(),
                group_order: GroupOrder::Original,
                content_exists: ContentExists::No,
                largest_location: None,
                forward: Forward::Enabled,
            };
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
            ];
            let l1 = b1.len() * 8;

            let v2 = Self {
                request_id: 9u8.into(),
                namespace: ["moq"].into(),
                name: "vod".into(),
                alias: 5u8.into(),
                group_order: GroupOrder::Original,
                content_exists: ContentExists::Yes,
                largest_location: Some((43u8, 15u8).into()),
                forward: Forward::Enabled,
            };
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
            ];
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(Publish);
}
