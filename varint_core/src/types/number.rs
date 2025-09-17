use std::fmt::{Debug, Display};

use bitvec::prelude::*;
use snafu::{ResultExt, Snafu};

use crate::{
    VarInt,
    io::{reader::ReaderError, writer::WriterError},
};

const MAX_U6: u64 = (2 << 5) - 1;
const MAX_U14: u64 = (2 << 13) - 1;
const MAX_U30: u64 = (2 << 29) - 1;
const MAX_U62: u64 = (2 << 61) - 1;

// x (i)
/// This Struct represents the primary
/// VarInt type of this crate. In the QUIC
/// and MOQT RFCs they are denoted by `x(i)`.
///
/// It has four possible sizes, identified
/// by the first 2 bits on the wire:
///
/// - `0b00`: next 6 bits are the number
/// - `0b01`: next 14 bits are the number
/// - `0b10`: next 30 bits are the number
/// - `0b11`: next 62 bits are the number
#[derive(Clone)]
pub struct Number {
    data: BitVec<u8>,
}

impl Number {
    /// Creates a new VarInt.
    ///
    /// This function only accept `u32` to provide
    /// and infallible constructor method.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::new(123);
    /// assert_eq!(v, 123);
    /// ```
    pub fn new(v: u32) -> Self {
        let mut this = Self::default();
        this.set_value(v.into()).expect("value will fit");
        this
    }

    /// Returns the value of the VarInt.
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::new(123);
    /// assert_eq!(v.value(), 123);
    /// ```
    pub fn value(&self) -> u64 {
        self.data.load_be()
    }

    /// Set the inner value to `v`.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let mut v = Number::default();
    /// v.set_value(15).unwrap();
    /// assert_eq!(v, 15);
    /// ```
    ///
    /// # Error
    ///
    /// This will return an Error when `v`
    /// is larger than a VarInt can be.
    ///
    /// Maximum: 4_611_686_018_427_387_903 (2^61-1)
    ///
    /// ```
    /// # use varint_core::Number;
    /// let mut v = Number::default();
    /// let err = v.set_value(u64::MAX);
    /// assert!(err.is_err());
    /// ```
    pub fn set_value(&mut self, v: u64) -> Result<&mut Self, NumberTooLarge> {
        snafu::ensure!(v < MAX_U62, NumberTooLargeSnafu { num: v });

        let len = super::num_bits(v);
        self.data.resize(len, false);
        self.data.store_be(v);

        Ok(self)
    }
}

impl Default for Number {
    fn default() -> Self {
        Self {
            data: bitvec!(u8, Lsb0; 0; 8),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VarInt")
            .field("value", &self.value())
            .field("num_bits", &super::num_bits(self.value()))
            .field("inner", &self.data)
            .finish()
    }
}

#[derive(Debug, Snafu, PartialEq, Clone)]
/// Error when [VarInt](crate::VarInt) implementation fails.
pub enum NumberError {
    ReaderError {
        source: ReaderError,
    },
    WriterError {
        source: WriterError,
    },

    /// empty buffer given to [decode](crate::VarInt::decode)
    #[snafu(display("unable to decode data from empty buffer"))]
    NoData,
    /// tried to create a VarInt with a too large number
    #[snafu(display("number >{num}< is too large"))]
    TooLarge {
        num: u64,
        source: NumberTooLarge,
    },
}

impl VarInt for Number {
    type Error = NumberError;
    type Item = Self;
    fn decode<R>(reader: &mut R) -> Result<Self::Item, Self::Error>
    where
        R: crate::Reader,
    {
        // first byte contains the size and (part
        // of) the number
        let byte = reader.read_bytes(1).context(ReaderSnafu)?[0];

        // first two bits denote how many bits are
        // part of the number
        let size = (byte & 0b1100_0000) >> 6;

        // start of the number
        let byte = byte & 0b0011_1111;

        let num = match size {
            0b00 => byte as u64,
            0b01 => {
                // one more byte needed
                let tail = reader.read_bytes(1).context(ReaderSnafu)?[0];

                u16::from_be_bytes([byte, tail]) as u64
            }
            0b10 => {
                // three more byte needed
                let tail = reader.read_bytes(3).context(ReaderSnafu)?;
                let mut buf = vec![byte];
                buf.append(&mut tail.into());

                u32::from_be_bytes(buf.try_into().expect("buf has len 4")) as u64
            }
            0b11 => {
                // seven more byte needed
                let tail = reader.read_bytes(7).context(ReaderSnafu)?;
                let mut buf = vec![byte];
                buf.append(&mut tail.into());

                u64::from_be_bytes(buf.try_into().expect("buf has len 8"))
            }
            _ => unreachable!("impossible size"),
        };

        // construct the VarInt
        let mut v = Self::default();
        v.set_value(num).context(TooLargeSnafu { num })?;
        Ok(v)
    }

    fn encode<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: crate::Writer,
    {
        let value = self.value();
        let buf = if value <= MAX_U6 {
            (value as u8).to_be_bytes().to_vec()
        } else if value <= MAX_U14 {
            (0b01 << 14 | (value as u16)).to_be_bytes().to_vec()
        } else if value <= MAX_U30 {
            (0b10 << 30 | (value as u32)).to_be_bytes().to_vec()
        } else {
            (0b11 << 62 | value).to_be_bytes().to_vec()
        };
        writer.write_bytes(&buf).context(WriterSnafu)?;
        Ok(())
    }
}

macro_rules! impl_partial_eq {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl PartialEq<$prim> for Number {
                fn eq(&self, other: &$prim) -> bool {
                    *other as u64 == self.value()
                }
            }
            impl PartialEq<Number> for $prim {
                fn eq(&self, other: &Number) -> bool {
                    *self as u64 == other.value()
                }
            }
        )+
    };
}
impl_partial_eq!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool
);

// TODO checks these again
macro_rules! impl_from {
    ( $typ:ty ; $($prim:ty),+ $(,)? ) => {
        $(
            impl From<$prim> for $typ {
                fn from(value: $prim) -> Self {
                    let mut v = Self::default();
                    v.set_value(value.into())
                        .expect("value cannot be larger than the maximum");
                    v
                }
            }
        )+
    };
}
impl_from!(Number; u8, u16, u32);

macro_rules! impl_try_from {
    ( $typ:ty ; $($prim:ty),+ $(,)? ) => {
        $(
            impl TryFrom<$prim> for $typ {
                type Error = ConversionError;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    let value = if value.lt(&value) {
                        return Err(ConversionError::TooLargeUnsigned { num: value as u128 });
                    } else {
                        value as i128
                    };
                    snafu::ensure!(value >= 0, NegativeSnafu { num: value });

                    let mut v = Number::default();
                    v.set_value(value as u64)
                        .context(TooLargeTODOSnafu { num: value })?;
                    Ok(v)
                }
            }
        )+
    };
}
impl_try_from!(Number; u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl From<Number> for u64 {
    fn from(value: Number) -> Self {
        value.value()
    }
}

#[derive(Debug, Snafu, PartialEq, Clone)]
pub enum ConversionError {
    Negative { num: i128 },
    TooLargeTODO { num: i128, source: NumberTooLarge },
    TooLargeUnsigned { num: u128 },
}

/// Number is larger than the maximum possible value of an VarInt
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(display("number >{num}< is too large to be an VarInt, max: {MAX_U62}"))]
pub struct NumberTooLarge {
    pub(crate) num: u64,
}

#[cfg(test)]
mod tests {
    use crate::{ReferenceReader, ReferenceWriter, Writer};

    use super::*;

    const VALID_U6_BUF: &[u8; 1] = &[0b0000_1000];
    const VALID_U14_BUF: &[u8; 2] = &[0b0100_1000, 0b0000_0000];
    const VALID_U30_BUF: &[u8; 4] = &[0b1000_0000, 0b0000_1000, 0b0000_0000, 0b0000_0000];
    const VALID_U62_BUF: &[u8; 8] = &[
        0b1100_0000,
        0b0000_1000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ];
    const VALID_NUM_U6: u8 = 8;
    const VALID_NUM_U14: u16 = 2_048;
    const VALID_NUM_U30: u32 = 524_288;
    const VALID_NUM_U62: u64 = 2_251_799_813_685_248;

    // TODO once Reader and Writer are working
    #[test]
    fn decode_test() {
        let buf = [
            VALID_U6_BUF.to_vec(),
            VALID_U14_BUF.to_vec(),
            VALID_U30_BUF.to_vec(),
            VALID_U62_BUF.to_vec(),
        ]
        .concat();
        let mut reader = ReferenceReader::new(&buf);

        let valid = Number::decode(&mut reader).unwrap();
        assert_eq!(valid, VALID_NUM_U6);

        let valid = Number::decode(&mut reader).unwrap();
        assert_eq!(valid, VALID_NUM_U14);

        let valid = Number::decode(&mut reader).unwrap();
        assert_eq!(valid, VALID_NUM_U30);

        let valid = Number::decode(&mut reader).unwrap();
        assert_eq!(valid, VALID_NUM_U62);
    }

    #[test]
    fn encode_test() {
        let buf = [
            VALID_U6_BUF.to_vec(),
            VALID_U14_BUF.to_vec(),
            VALID_U30_BUF.to_vec(),
            VALID_U62_BUF.to_vec(),
        ]
        .concat();
        let mut writer = ReferenceWriter::new();

        let num = Number::from(VALID_NUM_U6);
        num.encode(&mut writer).unwrap();

        let num = Number::from(VALID_NUM_U14);
        num.encode(&mut writer).unwrap();

        let num = Number::from(VALID_NUM_U30);
        num.encode(&mut writer).unwrap();

        let num = Number::try_from(VALID_NUM_U62).unwrap();
        num.encode(&mut writer).unwrap();

        assert_eq!(writer.finish().unwrap(), buf);
    }
}
