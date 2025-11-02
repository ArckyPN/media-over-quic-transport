use std::{fmt::Display, ops::Deref, vec::IntoIter};

use bytes::Bytes;
use snafu::{ResultExt, Snafu};

#[cfg(feature = "moq")]
use crate::BinaryData;
use crate::{
    Number, NumberError, VarInt,
    io::{reader::ReaderError, writer::WriterError},
    types::binary_data::BinaryDataError,
};

#[cfg(feature = "moq")]
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Tuple {
    data: Vec<BinaryData>,
}

impl Tuple {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bytes(&self) -> Vec<Bytes> {
        self.data.iter().map(|v| v.to_owned().into()).collect()
    }

    pub fn strings(&self) -> Vec<String> {
        self.data.iter().map(|v| v.to_string()).collect()
    }

    pub fn as_slice(&self) -> &[BinaryData] {
        &self.data
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

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl VarInt for Tuple {
    type Error = TupleError;
    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        let mut bits = 0;
        let (length, len) = Number::decode(reader, None).context(NumberSnafu)?;
        bits += len;

        let mut tuples = Vec::new();

        for _ in 0..length.number::<usize>() {
            let (data, b) = BinaryData::decode(reader, None).context(BinaryDataSnafu)?;
            bits += b;

            tuples.push(data);
        }

        Ok((tuples.into(), bits))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let len = self.data.len();
        let length = Number::try_new(len as u64).context(NumberSnafu)?;

        let mut bits = length.encode(writer, None).context(NumberSnafu)?;

        for tuple in &self.data {
            let b = tuple.encode(writer, None).context(BinaryDataSnafu)?;
            bits += b;
        }

        Ok(bits)
    }

    fn len_bits(&self) -> usize {
        self.data.iter().fold(0, |acc, b| acc + b.len_bits())
    }

    fn length_required() -> bool {
        // length is provided by the preceding VarInt
        false
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

#[derive(Debug, Snafu, Clone, PartialEq)]
pub enum TupleError {
    Number { source: NumberError<u64> },
    BinaryData { source: BinaryDataError },
    Reader { source: ReaderError },
    Writer { source: WriterError },
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::{ReferenceReader, ReferenceWriter, Writer};

    use super::*;

    const BUFFER: &[u8] = &[
        // x(i) = 2 tuples
        0b0000_0010,
        // x(i) = 3
        0b0000_0011,
        // 3 bytes, first BinaryData
        1,
        2,
        3,
        // x(i) = 5
        0b0000_0101,
        // 5 bytes, second BinaryData
        1,
        2,
        3,
        4,
        5,
    ];

    #[test]
    fn decode_test() {
        use pretty_assertions::assert_eq;
        let mut reader = ReferenceReader::new(BUFFER);

        let (tuple, bits) = Tuple::decode(&mut reader, None).unwrap();
        assert_eq!(bits, BUFFER.len() * 8);
        assert_eq!(tuple, Tuple::from([&BUFFER[2..5], &BUFFER[6..]]));
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let tuple = Tuple::from([&BUFFER[2..5], &BUFFER[6..]]);

        let bits = tuple.encode(&mut writer, None).unwrap();
        assert_eq!(bits, BUFFER.len() * 8);
        assert_eq!(writer.finish(), Ok(Bytes::from(BUFFER)));
    }
}
