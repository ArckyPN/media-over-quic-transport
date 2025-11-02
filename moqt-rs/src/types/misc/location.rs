use varint::{VarInt, draft_ref, funty::Unsigned, x};

/// ## Location
///
/// Identifies an Object by Group ID
/// and Object ID.
#[derive(Debug, VarInt, Clone, PartialEq)]
#[draft_ref(v = 14, rename = "name-location-structure")]
pub struct Location {
    group: x!(i),

    object: x!(i),
}

impl Location {
    pub fn new<G, O>(group: G, object: O) -> Self
    where
        G: Into<x!(i)>,
        O: Into<x!(i)>,
    {
        Self {
            group: group.into(),
            object: object.into(),
        }
    }

    // pub fn try_new<G, O>(group: G, object: O) -> Self
    // where
    //     G: TryInto<x!(i)>,
    //     O: TryInto<x!(i)>,
    // {
    //     Self {
    //         group: group.into(),
    //         object: object.into(),
    //     }
    // }
    // TODO try new with TryInto
    // TODO set_group/object? try_set_*?

    pub fn group<U>(&self) -> U
    where
        U: Unsigned,
    {
        self.group.number()
    }

    pub fn object<U>(&self) -> U
    where
        U: Unsigned,
    {
        self.object.number()
    }
}

impl<G, O> From<(G, O)> for Location
where
    G: Into<x!(i)>,
    O: Into<x!(i)>,
{
    fn from(value: (G, O)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}

impl<T> From<[T; 2]> for Location
where
    T: Into<x!(i)>,
{
    fn from(value: [T; 2]) -> Self {
        let [g, o] = value;
        Self::new(g.into(), o.into())
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Ref: https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#name-location-structure
        // A < B: A.Group < B.Group || (A.Group == B.Group && A.Object < B.Object)
        if self.group < other.group || (self.group == other.group && self.object < other.object) {
            return Some(std::cmp::Ordering::Less);
        } else if other.group < self.group
            || (other.group == self.group && other.object < self.object)
        {
            return Some(std::cmp::Ordering::Greater);
        }
        Some(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Location {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(15u8, 1u16);
            let b1 = vec![15, 1];
            let l1 = b1.len() * 8;

            let v2 = Self::from((1u32, 1u8));
            let b2 = vec![1, 1];
            let l2 = b2.len() * 8;

            let v3 = Self::from([9u8, 10u8]);
            let b3 = vec![9, 10];
            let l3 = b3.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3)]
        }
    }

    varint_struct_test!(Location);
}
