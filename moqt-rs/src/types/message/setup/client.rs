use varint::{VarInt, x};

/// TODO docs
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14, rename = "client_setup-and-server_set")]
pub struct ClientSetup {
    /// TODO docs
    #[varint(count = x(i))]
    supported_versions: x!(i; ...),
    // TODO parameters should be an extra type
}

impl ClientSetup {
    pub fn new<V>(versions: &[V]) -> Self
    where
        V: Into<x!(i)> + Clone,
    {
        Self {
            supported_versions: Vec::from_iter(versions.iter().map(|v| v.clone().into())),
        }
    }

    // TODO impls for usability
}

impl<V> FromIterator<V> for ClientSetup
where
    V: Into<x!(i)>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self {
            supported_versions: Vec::from_iter(iter.into_iter().map(Into::into)),
        }
    }
}

impl<T> From<Vec<T>> for ClientSetup
where
    T: Into<x!(i)>,
{
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}

impl<T> From<&'static [T]> for ClientSetup
where
    T: Into<x!(i)> + Clone,
{
    fn from(value: &'static [T]) -> Self {
        Self {
            supported_versions: Vec::from_iter(value.iter().map(|v| v.clone().into())),
        }
    }
}

impl<T> From<Box<[T]>> for ClientSetup
where
    T: Into<x!(i)>,
{
    fn from(value: Box<[T]>) -> Self {
        Self::from_iter(value)
    }
}

impl<T, const N: usize> From<[T; N]> for ClientSetup
where
    T: Into<x!(i)>,
{
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}

impl<T, const N: usize> From<&[T; N]> for ClientSetup
where
    T: Into<x!(i)> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        Self {
            supported_versions: Vec::from_iter(value.iter().map(|v| v.clone().into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for ClientSetup {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new(&[1u8, 2u8]);
            let b1 = vec![
                2, // num of supported version
                1, 2, // supported versions
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::from([1u8, 2u8, 3u8]);
            let b2 = vec![
                3, // num of supported version
                1, 2, 3, // supported versions
            ];
            let l2 = b2.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2)]
        }
    }

    varint_struct_test!(ClientSetup);
}
