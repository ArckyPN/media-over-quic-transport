mod error;
mod varint;

use {
    super::BinaryData,
    bytes::Bytes,
    std::{fmt::Display, ops::Deref, vec::IntoIter},
};

pub use error::TupleError;

pub(super) use error::ctx;

/// A Tuple or x(tuple) is a list
/// of [BinaryData].
///
/// For convenience you should use the [x!(tuple)](varint_derive::x) // TODO doc.rs link once varint_derive is published
/// macro instead of remembering this type.
///
/// It starts with a [VarInt](crate::Number) signaling
/// the number of [BinaryData] elements, followed by
/// that many [BinaryData].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tuple {
    data: Vec<BinaryData>,
}

impl Tuple {
    /// Returns the inner data as Byte Vector
    pub fn bytes(&self) -> Vec<Bytes> {
        self.data.iter().map(|v| v.to_owned().into()).collect()
    }

    /// Returns the inner data as String Vector
    pub fn strings(&self) -> Vec<String> {
        self.data.iter().map(|v| v.to_string()).collect()
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}
impl AsRef<Vec<BinaryData>> for Tuple {
    fn as_ref(&self) -> &Vec<BinaryData> {
        &self.data
    }
}

impl AsRef<[BinaryData]> for Tuple {
    fn as_ref(&self) -> &[BinaryData] {
        &self.data
    }
}

impl AsMut<Vec<BinaryData>> for Tuple {
    fn as_mut(&mut self) -> &mut Vec<BinaryData> {
        &mut self.data
    }
}

impl AsMut<[BinaryData]> for Tuple {
    fn as_mut(&mut self) -> &mut [BinaryData] {
        &mut self.data
    }
}

impl Deref for Tuple {
    type Target = [BinaryData];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl IntoIterator for Tuple {
    type Item = BinaryData;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tuple {
    type Item = &'a BinaryData;
    type IntoIter = core::slice::Iter<'a, BinaryData>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<V> FromIterator<V> for Tuple
where
    V: Into<BinaryData>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self {
            data: Vec::from_iter(iter.into_iter().map(Into::into)),
        }
    }
}

impl<T> From<Vec<T>> for Tuple
where
    T: Into<BinaryData>,
{
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}

impl<T> From<&'static [T]> for Tuple
where
    T: Into<BinaryData> + Clone,
{
    fn from(value: &'static [T]) -> Self {
        Self::from_iter(value.to_vec())
    }
}
impl<T> From<Box<[T]>> for Tuple
where
    T: Into<BinaryData>,
{
    fn from(value: Box<[T]>) -> Self {
        Self::from_iter(value)
    }
}

impl<T, const N: usize> From<[T; N]> for Tuple
where
    T: Into<BinaryData>,
{
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}

impl<T, const N: usize> From<&[T; N]> for Tuple
where
    T: Into<BinaryData> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        Self::from_iter(value.to_vec())
    }
}
