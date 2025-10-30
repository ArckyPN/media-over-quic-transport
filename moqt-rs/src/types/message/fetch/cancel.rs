use varint::{VarInt, x};

#[derive(Debug, VarInt, PartialEq, Clone)]
pub struct FetchCancel {
    request_id: x!(i),
}

impl FetchCancel {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<x!(i)>,
    {
        Self::from(id)
    }
}
// TODO impls for usability

impl<T> From<T> for FetchCancel
where
    T: Into<x!(i)>,
{
    fn from(value: T) -> Self {
        Self {
            request_id: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for FetchCancel {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = 5u8.into();
            let b1 = vec![5];
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(FetchCancel);
}
