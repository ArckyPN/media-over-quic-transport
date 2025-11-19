use bon::Builder;
use varint::VarInt;

use crate::types::{
    misc::Location,
    track::{Name, Namespace},
};

/// ## Standalone Fetch
///
/// Request past Objects from a Track within
/// a specified range.
///
/// [Fetch](crate::types::message::Fetch)
#[derive(Debug, VarInt, PartialEq, Clone, Builder)]
pub struct StandaloneFetch {
    /// ## Track Namespace
    ///
    /// The Namespace of the requested Track.
    #[builder(into, setters(
        doc {
            /// Sets the track namespace on [StandaloneFetch].
        }
    ))]
    pub namespace: Namespace,

    /// ## Track Name
    ///
    /// The Name of the requested Track.
    #[builder(into, setters(
        doc {
            /// Sets the track name on [StandaloneFetch].
        }
    ))]
    pub name: Name,

    /// ## First Object
    ///
    /// The ID of the Object to start with.
    #[builder(
        with = |group: impl Into<varint::x!(i)>, object: impl Into<varint::x!(i)>| (group.into(), object.into()).into(),
        setters(
        name = start,
        doc {
            /// Sets the start location on [StandaloneFetch].
        }
    ))]
    pub start_location: Location,

    /// ## Final Object
    ///
    /// The ID of the Object to end with.
    #[builder(
        with = |group: impl Into<varint::x!(i)>, object: impl Into<varint::x!(i)>| (group.into(), object.into()).into(),
        setters(
        name = end,
        doc {
            /// Sets the end location on [StandaloneFetch].
        }
    ))]
    pub end_location: Location,
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for StandaloneFetch {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .namespace(["test"])
                .name("ok")
                .start(1u8, 1u8)
                .end(15u8, 10u8)
                .build();
            let b1 = vec![
                1, // namespace len 1
                4, // tuple len 4
                b't', b'e', b's', b't', // tuple "test"
                2,    // name len 2
                b'o', b'k', // name "ok"
                1,    // start group 1
                1,    // start object 1
                15,   // end group 15
                10,   // end object 10
            ];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(StandaloneFetch);
}
