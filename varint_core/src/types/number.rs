use std::fmt::{Debug, Display};

use bitvec::prelude::*;
use funty::{AtMost32, Integral, Unsigned};
use snafu::{ResultExt, Snafu};

use crate::{
    VarInt,
    io::{reader::ReaderError, writer::WriterError},
};

const MAX_U6: u64 = (2 << 5) - 1;
const MAX_U14: u64 = (2 << 13) - 1;
const MAX_U30: u64 = (2 << 29) - 1;
const MAX_U62: u64 = (2 << 61) - 1;

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
#[derive(Clone, PartialEq)]
pub struct Number {
    data: BitVec<u8>,
}

impl Number {
    /// Creates a new VarInt.
    ///
    /// This function only accept `u8`, `u16`
    /// and `u32` to provide and infallible
    /// constructor method.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::new(123u32);
    /// assert_eq!(v, 123);
    /// ```
    pub fn new<I>(v: I) -> Self
    where
        I: Unsigned + AtMost32,
    {
        let mut this = Self::default();
        this.set_value(v).expect("value will fit");
        this
    }

    /// Tries to construct a new VarInt from any
    /// unsigned integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::try_new(123u64).unwrap();
    /// assert_eq!(v, 123);
    /// ```
    pub fn try_new<I>(v: I) -> Result<Self, NumberError>
    where
        I: Unsigned,
    {
        let mut this = Self::default();
        this.set_value(v)?;
        Ok(this)
    }

    /// Returns the value of the VarInt.
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::new(123u32);
    /// assert_eq!(v.value::<u32>(), 123);
    /// ```
    ///
    /// Tip: using `value::<u64>()` will
    /// ensure to always get the full value.
    pub fn value<I>(&self) -> I
    where
        I: Unsigned,
    {
        self.data.load_be()
    }

    /// Set the inner value to `v`.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let mut v = Number::default();
    /// v.set_value(15u8).unwrap();
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
    pub fn set_value<I>(&mut self, v: I) -> Result<&mut Self, NumberError>
    where
        I: Unsigned,
    {
        snafu::ensure!(
            v.as_u128() <= (MAX_U62 as u128),
            TooLargeSnafu { num: v.as_u128() }
        );

        let len = super::num_bits(v);
        self.data.resize(len as usize, false);
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
        write!(f, "{}", self.value::<u64>())
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.value::<u64>();
        f.debug_struct("VarInt")
            .field("value", &v)
            .field("num_bits", &super::num_bits(v))
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
    /// tried to create a VarInt with a too large number
    #[snafu(display("number >{num}< is too large"))]
    TooLarge {
        num: u128,
    },
}

impl VarInt for Number {
    type Error = NumberError;
    fn decode<R>(reader: &mut R) -> Result<Self, Self::Error>
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
        v.set_value(num)?;
        Ok(v)
    }

    fn encode<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: crate::Writer,
    {
        let value = self.value::<u64>();
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
        // u64 is largest type Number can be
        // forcing
        $(
            impl PartialEq<$prim> for Number {
                fn eq(&self, other: &$prim) -> bool {
                    if *other < (0 as $prim) { return false; }
                    if (*other as u64) > MAX_U62 { return false; }
                    // now that it is verified that other is
                    // not negative or larger than allowed
                    // it is permissible to case both to u64
                    *other as u64 == self.value::<u64>()
                }
            }
            impl PartialEq<Number> for $prim {
                fn eq(&self, other: &Number) -> bool {
                    // flip the comparison to make use of
                    // the partial eq above
                    other == self
                }
            }
        )+
    };
}
impl_partial_eq!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_partial_ord    {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl PartialOrd<$prim> for Number {
                fn partial_cmp(&self, other: &$prim) -> Option<std::cmp::Ordering> {
                    if *other < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // other is positive -> casting to u64 is fine
                    let other = *other as u64;
                    if self.value::<u64>() > other {
                        Some(std::cmp::Ordering::Greater)
                    } else if self.value::<u64>() < other {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }
            }
            impl PartialOrd<Number> for $prim {
                fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
                    if *self < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // self is positive -> casting to u64 is fine
                    if (*self as u64) > other.value::<u64>() {
                        Some(std::cmp::Ordering::Greater)
                    } else if (*self as u64) < other.value::<u64>() {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }
            }
        )+
    };
}
impl_partial_ord!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl From<$prim> for Number {
                fn from(value: $prim) -> Self {
                    Self::new(value)
                }
            }
        )+
    };
}
impl_from!(u8, u16, u32);

macro_rules! impl_from_number {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl From<Number> for $prim {
                fn from(value: Number) -> Self {
                    value.value()
                }
            }
        )+
    };
}
impl_from_number!(u64, u128);

macro_rules! impl_try_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl TryFrom<$prim> for Number {
                type Error = ConversionError<$prim>;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(value >= (0 as $prim), IsNegativeSnafu { value });
                    // value is verified to be positive => casting
                    // to u64 is permissible
                    Number::try_new(value as u64).context(InvalidSnafu { value })
                }
            }
        )+
    };
}
impl_try_from!(u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_try_from_number {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl TryFrom<Number> for $prim {
                type Error = ConversionError<$prim>;
                fn try_from(value: Number) -> Result<Self, Self::Error> {
                    snafu::ensure!(value <= <$prim>::MAX, UnFitSnafu { value, max: <$prim>::MAX });

                    Ok(value.value::<u64>() as $prim)
                }
            }
        )+
    };
}
impl_try_from_number!(u8, u16, u32, usize, i8, i16, i32, i64, i128, isize);

/// Error when casting to and from [Number] to primitive types.
#[derive(Debug, Snafu, PartialEq, Clone)]
pub enum ConversionError<I>
where
    I: Integral,
{
    /// Error when trying to create a [Number] from a negative integer
    #[snafu(display("VarInt Number cannot be negative, trying >{value}<"))]
    IsNegative { value: I },
    /// Error when the number was too large
    #[snafu(display("failed to create a VarInt Number from >{value}<"))]
    Invalid { value: I, source: NumberError },
    /// Error when trying to cast a Number into a too small type
    #[snafu(display("Number >{value}< does not fit into the given type, max value: >{max}<"))]
    UnFit { value: Number, max: I },
}

/// Number is larger than the maximum possible value of an VarInt
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(display("number >{num}< is too large to be an VarInt, max: {MAX_U62}"))]
pub struct NumberTooLarge<I>
where
    I: Integral,
{
    pub(crate) num: I,
}

#[cfg(test)]
mod tests {
    use crate::{ReferenceReader, ReferenceWriter, Writer};

    use super::*;

    #[test]
    fn set_value_test() {
        let mut base = Number::default();

        base.set_value(8u8).unwrap();
        assert_eq!(base, 8);

        base.set_value(700u16).unwrap();
        assert_eq!(base, 700);

        base.set_value(2_123_789u32).unwrap();
        assert_eq!(base, 2_123_789);

        base.set_value(MAX_U62).unwrap();
        assert_eq!(base, MAX_U62);

        let err = base.set_value(MAX_U62 + 1).unwrap_err();
        assert_eq!(
            err,
            NumberError::TooLarge {
                num: (MAX_U62 + 1) as u128
            }
        );
    }

    #[test]
    fn new_test() {
        let valid = Number::new(15u8);
        assert_eq!(valid.value::<u8>(), 15u8);
        assert_eq!(valid.data.load_be::<u16>(), 15);

        let valid = Number::new(537u16);
        assert_eq!(valid.data.load_be::<u16>(), 537);

        let valid = Number::new(2_123_789u32);
        assert_eq!(valid.data.load_be::<u32>(), 2_123_789);
    }

    #[test]
    fn try_new_test() {
        let valid = Number::try_new(15u8).unwrap();
        assert_eq!(valid, 15);

        let valid = Number::try_new(9_000_000_000u64).unwrap();
        assert_eq!(valid, 9_000_000_000u64);

        let valid = Number::try_new(MAX_U62).unwrap();
        assert_eq!(valid, MAX_U62);

        let valid = Number::try_new(MAX_U62 + 1).unwrap_err();
        assert_eq!(
            valid,
            NumberError::TooLarge {
                num: (MAX_U62 + 1) as u128
            }
        );
    }

    #[test]
    fn eq_test() {
        assert_eq!(Number::new(123u8), 123);
        assert_eq!(Number::new(537u16), 537);
        assert_eq!(Number::new(2_123_789u32), 2_123_789);

        assert_ne!(Number::new(123u8), -5);
        assert_ne!(Number::new(234u8), i128::MIN);
        assert_ne!(Number::new(123u16), i64::MAX);
    }

    #[test]
    fn ord_test() {
        let num = Number::new(123u8);
        assert!(num > 50u8);
        assert!(50u64 < num);
        assert!(num > 13i8);
        assert!(num < 250u16);
        assert!(250u16 > num);
        assert!(num == 123u64);
        assert!(123u8 == num);
    }

    #[test]
    fn default_test() {
        assert_eq!(Number::default(), 0);
    }

    #[test]
    fn from_test() {
        let num = Number::from(40u8);
        assert_eq!(num, 40);
        assert_eq!(u64::from(num.clone()), 40);
        assert_eq!(u64::from(num), 40);

        let num = Number::from(537u16);
        assert_eq!(num, 537);

        let num = Number::from(2_223_789_999u32);
        assert_eq!(num, 2_223_789_999u64);
        assert_eq!(
            i32::try_from(num.clone()).unwrap_err(),
            ConversionError::UnFit {
                value: num,
                max: i32::MAX
            }
        );
    }

    #[test]
    fn try_from_test() {
        let valid = Number::try_from(9_000_000_000u64).unwrap();
        assert_eq!(valid, 9_000_000_000u64);

        let valid = Number::try_from(MAX_U62).unwrap();
        assert_eq!(valid, MAX_U62);

        let invalid = Number::try_from(MAX_U62 + 1).unwrap_err();
        assert_eq!(
            invalid,
            ConversionError::Invalid {
                value: MAX_U62 + 1,
                source: NumberError::TooLarge {
                    num: (MAX_U62 + 1) as u128
                }
            }
        );

        let invalid = Number::try_from(-1).unwrap_err();
        assert_eq!(invalid, ConversionError::IsNegative { value: -1 });

        let num = Number::new(537u16);
        assert_eq!(
            u8::try_from(num.clone()).unwrap_err(),
            ConversionError::UnFit {
                value: num,
                max: u8::MAX
            }
        );

        let num = Number::new(2_223_789_999u32);
        assert_eq!(
            i32::try_from(num.clone()).unwrap_err(),
            ConversionError::UnFit {
                value: num,
                max: i32::MAX
            }
        );
    }

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
