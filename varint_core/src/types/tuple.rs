use std::fmt::Display;

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
        self.data.iter().map(|v| v.to_owned().into()).collect()
    }

    pub fn as_slice(&self) -> &[BinaryData] {
        &self.data
    }
}

// TODO impl Vec and slice stuff for easier usage

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

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl VarInt for Tuple {
    type Error = TupleError;
    fn decode<R>(reader: &mut R, length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        let (length, mut bits) = match length {
            Some(l) => (l, l * 8),
            None => {
                let (num, bits) = Number::decode(reader, None).context(NumberSnafu)?;

                let num = num.number();

                (num, bits)
            }
        };

        let mut tuples = Vec::new();

        for _ in 0..length {
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
}

impl From<Vec<BinaryData>> for Tuple {
    fn from(value: Vec<BinaryData>) -> Self {
        Self { data: value }
    }
}

impl From<Vec<&[u8]>> for Tuple {
    fn from(value: Vec<&[u8]>) -> Self {
        Self {
            data: value.iter().map(|v| v.to_vec().into()).collect(),
        }
    }
}

impl From<&[BinaryData]> for Tuple {
    fn from(value: &[BinaryData]) -> Self {
        Self {
            data: value.to_vec(),
        }
    }
}

impl<const N: usize> From<[BinaryData; N]> for Tuple {
    fn from(value: [BinaryData; N]) -> Self {
        Self {
            data: value.to_vec(),
        }
    }
}

impl From<&[&[u8]]> for Tuple {
    fn from(value: &[&[u8]]) -> Self {
        Self {
            data: value.iter().map(|v| v.to_vec().into()).collect(),
        }
    }
}

impl<const N: usize, const M: usize> From<[[u8; M]; N]> for Tuple {
    fn from(value: [[u8; M]; N]) -> Self {
        Self {
            data: value.iter().map(|v| v.to_vec().into()).collect(),
        }
    }
}

impl<const N: usize> From<[&[u8]; N]> for Tuple {
    fn from(value: [&[u8]; N]) -> Self {
        Self {
            data: value.iter().map(|v| v.to_vec().into()).collect(),
        }
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
