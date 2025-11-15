mod error;
mod varint;
mod varint_bytes;

use {
    bytes::{Bytes, BytesMut, buf::IntoIter},
    std::{
        fmt::{Debug, Display},
        hash::Hash,
    },
};

pub use error::BinaryDataError;

pub(super) use error::ctx;

/// A BinaryData is a sequence of Bytes.
///
/// For convenience you should use the [x!(i)](varint_derive::x) // TODO doc.rs link once varint_derive is published
/// macro instead of remembering this type.
///
/// It starts with a [VarInt](crate::Number), followed
/// by that many bytes.
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct BinaryData {
    data: Bytes,
}

impl Display for BinaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self))
    }
}

impl Debug for BinaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryData")
            .field("len", &self.data.len())
            .field("data", &self.data.to_vec())
            .finish()
    }
}

// more or less the same impls as bytes::Bytes

impl core::ops::Deref for BinaryData {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl AsRef<[u8]> for BinaryData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Hash for BinaryData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl core::borrow::Borrow<[u8]> for BinaryData {
    fn borrow(&self) -> &[u8] {
        &self.data
    }
}

impl IntoIterator for BinaryData {
    type Item = u8;
    type IntoIter = IntoIter<Bytes>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.data)
    }
}

impl<'a> IntoIterator for &'a BinaryData {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl FromIterator<u8> for BinaryData {
    fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self {
        Vec::from_iter(into_iter).into()
    }
}

impl PartialEq<[u8]> for BinaryData {
    fn eq(&self, other: &[u8]) -> bool {
        self.data == other
    }
}

impl PartialOrd<[u8]> for BinaryData {
    fn partial_cmp(&self, other: &[u8]) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl PartialEq<BinaryData> for [u8] {
    fn eq(&self, other: &BinaryData) -> bool {
        *other == *self
    }
}

impl PartialOrd<BinaryData> for [u8] {
    fn partial_cmp(&self, other: &BinaryData) -> Option<std::cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self, other)
    }
}

impl PartialEq<str> for BinaryData {
    fn eq(&self, other: &str) -> bool {
        self.data == other.as_bytes()
    }
}

impl PartialOrd<str> for BinaryData {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other.as_bytes())
    }
}

impl PartialEq<BinaryData> for str {
    fn eq(&self, other: &BinaryData) -> bool {
        *other == *self
    }
}

impl PartialOrd<BinaryData> for str {
    fn partial_cmp(&self, other: &BinaryData) -> Option<std::cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self.as_bytes(), other)
    }
}

impl PartialEq<Vec<u8>> for BinaryData {
    fn eq(&self, other: &Vec<u8>) -> bool {
        *self == other[..]
    }
}

impl PartialOrd<Vec<u8>> for BinaryData {
    fn partial_cmp(&self, other: &Vec<u8>) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other[..])
    }
}

impl PartialEq<BinaryData> for Vec<u8> {
    fn eq(&self, other: &BinaryData) -> bool {
        *other == *self
    }
}

impl PartialOrd<BinaryData> for Vec<u8> {
    fn partial_cmp(&self, other: &BinaryData) -> Option<std::cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self, other)
    }
}

impl PartialEq<String> for BinaryData {
    fn eq(&self, other: &String) -> bool {
        *self == other[..]
    }
}

impl PartialOrd<String> for BinaryData {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other.as_bytes())
    }
}

impl PartialEq<BinaryData> for String {
    fn eq(&self, other: &BinaryData) -> bool {
        other.data == *self
    }
}

impl PartialOrd<BinaryData> for String {
    fn partial_cmp(&self, other: &BinaryData) -> Option<std::cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self.as_bytes(), other)
    }
}

impl PartialEq<BinaryData> for &[u8] {
    fn eq(&self, other: &BinaryData) -> bool {
        other.data == *self
    }
}

impl PartialOrd<BinaryData> for &[u8] {
    fn partial_cmp(&self, other: &BinaryData) -> Option<std::cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self, other)
    }
}

impl PartialEq<BinaryData> for &str {
    fn eq(&self, other: &BinaryData) -> bool {
        other.data == *self
    }
}

impl PartialOrd<BinaryData> for &str {
    fn partial_cmp(&self, other: &BinaryData) -> Option<std::cmp::Ordering> {
        <[u8] as PartialOrd<[u8]>>::partial_cmp(self.as_bytes(), other)
    }
}

impl PartialEq<BytesMut> for BinaryData {
    fn eq(&self, other: &BytesMut) -> bool {
        self.data == *other
    }
}

impl PartialEq<BinaryData> for BytesMut {
    fn eq(&self, other: &BinaryData) -> bool {
        *self == other.data
    }
}

impl From<&'static [u8]> for BinaryData {
    fn from(value: &'static [u8]) -> Self {
        Self {
            data: Bytes::from_static(value),
        }
    }
}

impl From<&'static str> for BinaryData {
    fn from(value: &'static str) -> Self {
        Self { data: value.into() }
    }
}

impl From<Box<[u8]>> for BinaryData {
    fn from(value: Box<[u8]>) -> Self {
        Self { data: value.into() }
    }
}

impl From<String> for BinaryData {
    fn from(value: String) -> Self {
        Self { data: value.into() }
    }
}

macro_rules! impl_from {
    ( $($typ:path),+ $(,)* ) => {
        $(
            impl From<$typ> for BinaryData {
                fn from(value: $typ) -> Self {
                    Self {
                        data: value.into(),
                    }
                }
            }
            impl From<&$typ> for BinaryData {
                fn from(value: &$typ) -> Self {
                    Self {
                        data: value.clone().into(),
                    }
                }
            }
            impl From<BinaryData> for $typ {
                fn from(value: BinaryData) -> Self {
                    <$typ>::from(value.data)
                }
            }
            impl From<&BinaryData> for $typ {
                fn from(value: &BinaryData) -> Self {
                    <$typ>::from(value.data.clone())
                }
            }
        )+
    };
}
impl_from!(Vec<u8>, Bytes, BytesMut);
