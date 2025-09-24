use std::fmt::{Debug, Display};

use funty::{AtMost32, Integral, Unsigned};
use snafu::{ResultExt, Snafu};

use crate::{
    VarInt,
    bitstore::{self, BitStore},
    io::{reader::ReaderError, writer::WriterError},
};

const MAX_U6: u64 = (2 << 5) - 1;
const MAX_U14: u64 = (2 << 13) - 1;
const MAX_U30: u64 = (2 << 29) - 1;
const MAX_U62: u64 = (2 << 61) - 1;

/// This type represents the primary
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
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Number {
    data: BitStore<6, 62>,
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
    pub fn new<U>(v: U) -> Self
    where
        U: Unsigned + AtMost32,
    {
        let mut this = Self::default();
        this.set_number(v).expect("value will fit");
        this
    }

    /// Tries to construct a new VarInt from any
    /// unsigned integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::try_new(123u64);
    /// assert_eq!(v, 123);
    /// ```
    pub fn try_new<U>(v: U) -> Result<Self, NumberError<U>>
    where
        U: Unsigned,
    {
        let mut this = Self::default();
        this.set_number(v)?;
        Ok(this)
    }

    /// Returns the value of the VarInt.
    ///
    /// ```
    /// # use varint_core::Number;
    /// let v = Number::new(123u32);
    /// assert_eq!(v.number::<u32>(), 123);
    /// ```
    ///
    /// Tip: using `number::<u64>()` will
    /// ensure to always get the full value.
    pub fn number<U>(&self) -> U
    where
        U: Unsigned,
    {
        self.data.number()
    }

    /// Set the inner value to `v`.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::Number;
    /// let mut v = Number::default();
    /// v.set_number(15u8);
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
    /// let err = v.set_number(u64::MAX);
    /// assert!(err.is_err());
    /// ```
    pub fn set_number<U>(&mut self, v: U) -> Result<&mut Self, NumberError<U>>
    where
        U: Unsigned,
    {
        snafu::ensure!(v.as_u128() <= (MAX_U62 as u128), TooLargeSnafu { num: v });

        let len = match v {
            x if x.as_u64() <= MAX_U6 => 6,
            x if x.as_u64() <= MAX_U14 => 14,
            x if x.as_u64() <= MAX_U30 => 30,
            x if x.as_u64() <= MAX_U62 => 62,
            _ => unreachable!("number cannot be larger than (2 << 61) - 1"),
        };
        self.data.set_number(v, Some(len)).context(BitStoreSnafu)?;

        Ok(self)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number::<u64>())
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.number::<u64>();
        f.debug_struct("VarInt")
            .field("value", &v)
            .field("num_bits", &super::num_bits(v))
            .field("inner", &self.data)
            .finish()
    }
}

impl VarInt for Number {
    type Error = NumberError<u64>;
    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        // first byte contains the size and (part
        // of) the number
        let byte = reader.read_bytes(1).context(ReaderSnafu)?[0];

        // first two bits denote how many bits are
        // part of the number
        let size = (byte & 0b1100_0000) >> 6;

        // start of the number
        let byte = byte & 0b0011_1111;

        let mut vec = vec![byte];
        let bits = match size {
            0b00 => 6,
            0b01 => {
                // one more byte needed
                let ext = reader.read_bytes(1).context(ReaderSnafu)?;
                vec.extend_from_slice(&ext);
                14
            }
            0b10 => {
                // three more byte needed
                let ext = reader.read_bytes(3).context(ReaderSnafu)?;
                vec.extend_from_slice(&ext);
                30
            }
            0b11 => {
                // seven more byte needed
                let ext = reader.read_bytes(7).context(ReaderSnafu)?;
                vec.extend_from_slice(&ext);
                62
            }
            _ => unreachable!("impossible size"),
        };

        // shift left by 2 to account for the 2 size bits
        let len = vec.len();
        for i in 0..len {
            vec[i] <<= 2;
            if i == len - 1 {
                continue;
            }
            vec[i] += vec[i + 1] >> 6;
        }

        // construct the VarInt
        let mut v = Self::default();
        v.data.set_bits(&vec, bits).context(BitStoreSnafu)?;
        Ok((v, bits + 2))
    }

    fn encode<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: crate::Writer,
    {
        let value = self.number::<u64>();
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

/// Error when [VarInt](crate::VarInt) implementation fails.
#[derive(Debug, Snafu, PartialEq, Clone)]
pub enum NumberError<U>
where
    U: Unsigned,
{
    ReaderError {
        source: ReaderError,
    },
    WriterError {
        source: WriterError,
    },
    #[snafu(display("unable to store"))]
    BitStoreError {
        source: bitstore::Error,
    },
    /// tried to create a VarInt with a too large number
    #[snafu(display("number >{num}< is too large"))]
    TooLarge {
        num: U,
    },
}

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
    Invalid { value: I, source: NumberError<u64> },
    /// Error when trying to cast a Number into a too small type
    #[snafu(display("Number >{value}< does not fit into the given type, max value: >{max}<"))]
    UnFit { value: Number, max: I },
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
                    *other as u64 == self.number::<u64>()
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
                    if self.number::<u64>() > other {
                        Some(std::cmp::Ordering::Greater)
                    } else if self.number::<u64>() < other {
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
                    if (*self as u64) > other.number::<u64>() {
                        Some(std::cmp::Ordering::Greater)
                    } else if (*self as u64) < other.number::<u64>() {
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
                    value.number()
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

                    Ok(value.number::<u64>() as $prim)
                }
            }
        )+
    };
}
impl_try_from_number!(u8, u16, u32, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::{ReferenceReader, ReferenceWriter, Writer};

    use super::*;

    #[test]
    fn default_test() {
        assert_eq!(Number::default(), 0);
    }

    #[test]
    fn set_number_test() {
        let mut base = Number::default();

        let valid = base.set_number(8u8);
        assert!(valid.is_ok());
        assert_eq!(base, 8);

        let valid = base.set_number(700u16);
        assert!(valid.is_ok());
        assert_eq!(base, 700);

        let valid = base.set_number(2_123_789u32);
        assert!(valid.is_ok());
        assert_eq!(base, 2_123_789);

        let valid = base.set_number(MAX_U62);
        assert!(valid.is_ok());
        assert_eq!(base, MAX_U62);

        let err = base.set_number(MAX_U62 + 1);
        assert_eq!(err, Err(NumberError::TooLarge { num: MAX_U62 + 1 }));
    }

    #[test]
    fn new_test() {
        let valid = Number::new(15u8);
        assert_eq!(valid.number::<u8>(), 15u8);
        assert_eq!(valid.data.number::<u16>(), 15);

        let valid = Number::new(537u16);
        assert_eq!(valid.data.number::<u16>(), 537);

        let valid = Number::new(2_123_789u32);
        assert_eq!(valid.data.number::<u32>(), 2_123_789);
    }

    #[test]
    fn try_new_test() {
        let valid = Number::try_new(15u8);
        assert_eq!(valid, Ok(Number::new(15u16)));

        let valid = Number::try_new(9_000_000_000u64);
        assert_eq!(valid.map(|n| n.number::<u64>()), Ok(9_000_000_000u64));

        let valid = Number::try_new(MAX_U62);
        assert_eq!(valid.map(|n| n.number::<u64>()), Ok(MAX_U62));

        let valid = Number::try_new(MAX_U62 + 1);
        assert_eq!(valid, Err(NumberError::TooLarge { num: MAX_U62 + 1 }));
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
    fn from_test() {
        let num = Number::from(40u8);
        assert_eq!(num, 40);
        assert_eq!(u64::from(num.clone()), 40);
        assert_eq!(u64::from(num), 40);

        let num = Number::from(537u16);
        assert_eq!(num, 537);

        let num = Number::from(2_223_789_999u32);
        assert_eq!(num, 2_223_789_999u64);

        let invalid = i32::try_from(num.clone());
        assert_eq!(
            invalid,
            Err(ConversionError::UnFit {
                value: num,
                max: i32::MAX
            })
        );
    }

    #[test]
    fn try_from_test() {
        let Ok(valid) = Number::try_from(9_000_000_000u64) else {
            unreachable!("is valid u62 number")
        };
        assert_eq!(valid, 9_000_000_000u64);

        let Ok(valid) = Number::try_from(MAX_U62) else {
            unreachable!("is valid u62 number")
        };
        assert_eq!(valid, MAX_U62);

        let invalid = Number::try_from(MAX_U62 + 1);
        assert_eq!(
            invalid,
            Err(ConversionError::Invalid {
                value: MAX_U62 + 1,
                source: NumberError::TooLarge { num: MAX_U62 + 1 }
            })
        );

        let invalid = Number::try_from(-1);
        assert_eq!(invalid, Err(ConversionError::IsNegative { value: -1 }));

        let num = Number::new(537u16);
        let invalid = u8::try_from(num.clone());
        assert_eq!(
            invalid,
            Err(ConversionError::UnFit {
                value: num,
                max: u8::MAX
            })
        );

        let num = Number::new(2_223_789_999u32);
        let invalid = i32::try_from(num.clone());
        assert_eq!(
            invalid,
            Err(ConversionError::UnFit {
                value: num,
                max: i32::MAX
            })
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

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::from(VALID_NUM_U6), 8)));

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::from(VALID_NUM_U14), 16)));

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::from(VALID_NUM_U30), 32)));

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::try_from(VALID_NUM_U62).unwrap(), 64)));
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
        let valid = num.encode(&mut writer);
        assert_eq!(valid, Ok(()));

        let num = Number::from(VALID_NUM_U14);
        let valid = num.encode(&mut writer);
        assert_eq!(valid, Ok(()));

        let num = Number::from(VALID_NUM_U30);
        let valid = num.encode(&mut writer);
        assert_eq!(valid, Ok(()));

        let Ok(num) = Number::try_from(VALID_NUM_U62) else {
            unreachable!("valid u62 number")
        };
        let valid = num.encode(&mut writer);
        assert_eq!(valid, Ok(()));

        let valid = writer.finish();
        assert_eq!(valid, Ok(Bytes::from(buf)));
    }
}
