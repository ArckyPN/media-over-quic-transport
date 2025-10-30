use varint::VarInt;

use crate::types::{location::Location, track};

#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct StandaloneFetch {
    namespace: track::Namespace,
    name: track::Name,
    start_location: Location,
    end_location: Location,
}

// TODO impls for usability
impl StandaloneFetch {
    pub fn new<S, N, LS, LE>(namespace: S, name: N, start: LS, end: LE) -> Self
    where
        S: Into<track::Namespace>,
        N: Into<track::Name>,
        LS: Into<Location>,
        LE: Into<Location>,
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            start_location: start.into(),
            end_location: end.into(),
        }
    }

    // TODO try new with TryInto instead
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for StandaloneFetch {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(["test"], "ok", (1u8, 1u8), (15u8, 10u8));
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
