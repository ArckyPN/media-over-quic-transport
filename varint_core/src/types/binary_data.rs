use core::hash;
use std::fmt::Display;

use bytes::{Bytes, BytesMut, buf::IntoIter};
use snafu::{ResultExt, Snafu};

use crate::{
    Number, NumberError, VarInt,
    io::{reader::ReaderError, writer::WriterError},
};

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct BinaryData {
    data: Bytes,
}

impl BinaryData {
    pub fn new(buf: &[u8]) -> Self {
        buf.to_vec().into()
    }

    pub fn bytes(&self) -> Bytes {
        self.data.clone()
    }

    pub fn string(&self) -> String {
        String::from_utf8_lossy(self).to_string()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

// TODO impl Vec and slice stuff for easier usage

impl Display for BinaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self))
    }
}

impl VarInt for BinaryData {
    type Error = BinaryDataError;
    fn decode<R>(reader: &mut R, length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        let (length, bits) = match length {
            Some(l) => (l, l * 8),
            None => {
                let (num, bits) = Number::decode(reader, None).context(NumberSnafu)?;

                let num = num.number();

                (num, bits + num * 8)
            }
        };

        let data = reader.read_bytes(length).context(ReaderSnafu)?;

        Ok((data.into(), bits))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let len = self.data.len();
        let length = Number::try_new(len as u64).context(NumberSnafu)?;

        let mut bits = length.encode(writer, None).context(NumberSnafu)?;

        writer.write_bytes(&self.data).context(WriterSnafu)?;
        bits += len * 8;

        Ok(bits)
    }
}

#[derive(Debug, Snafu, PartialEq, Clone)]
pub enum BinaryDataError {
    Number { source: NumberError<u64> },
    Reader { source: ReaderError },
    Writer { source: WriterError },
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

impl hash::Hash for BinaryData {
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

impl From<BinaryData> for String {
    fn from(value: BinaryData) -> Self {
        String::from_utf8_lossy(&value).to_string()
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
            impl From<BinaryData> for $typ {
                fn from(value: BinaryData) -> Self {
                    <$typ>::from(value.data)
                }
            }
        )+
    };
}
impl_from!(Vec<u8>, Bytes, BytesMut);

#[cfg(test)]
mod tests {
    use crate::{ReferenceReader, ReferenceWriter, Writer};

    use super::*;

    const BUFFER: &[u8] = &[
        // x(i) => 8
        0b0000_1000,
        // 8 bytes data
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    ];
    const INVALID: &[u8] = &[
        // x(i) => 8
        0b0000_1000,
        // only 7 bytes data => too few
        1,
        2,
        3,
        4,
        5,
        6,
        7,
    ];

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUFFER);

        let (valid, bits) = BinaryData::decode(&mut reader, None).unwrap();
        assert_eq!(bits, BUFFER.len() * 8);
        assert_eq!(valid.data, BUFFER[1..]);

        let mut reader = ReferenceReader::new(INVALID);

        let invalid = BinaryData::decode(&mut reader, None);
        assert_eq!(
            invalid,
            Err(BinaryDataError::Reader {
                source: ReaderError::MissingBytes { needs: 8, left: 7 }
            })
        );
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let data = BinaryData::new(&BUFFER[1..]);
        let bits = data.encode(&mut writer, None).unwrap();

        assert_eq!(bits, BUFFER.len() * 8);
        assert_eq!(writer.finish(), Ok(Bytes::from(BUFFER)));
    }
}
