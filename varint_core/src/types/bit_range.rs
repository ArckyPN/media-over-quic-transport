use std::fmt::Debug;

use bytes::Bytes;
use funty::{Integral, Unsigned};
use snafu::{ResultExt, Snafu};

use crate::{
    VarInt,
    bitstore::{self, BitStore},
    io::reader::ReaderError,
};

// TODO in the proc macro make sure this ends on a bytes boundary, current num bits % 8 == 0
/// This type represents the type `x (A..B)`
/// in the QUIC RFC.
///
/// It is a Number that has a binary
/// representation with a number of
/// bits between `A` and `B`. It always
/// ends on a byte boundary.
///
/// `A` and `B` can both be omitted to
/// open their respective range limits.
#[derive(Default, Clone, PartialEq, PartialOrd)]
pub struct BitRange<const MIN: usize = 0, const MAX: usize = { usize::MAX }> {
    data: BitStore<MIN, MAX>,
}

impl<const MIN: usize, const MAX: usize> BitRange<MIN, MAX> {
    // TODO doc + example
    pub fn new_number<U>(v: U, n: Option<usize>) -> Result<Self, BitRangeError<U>>
    where
        U: Unsigned,
    {
        let mut this = Self::default();
        this.set_number(v, n)?;
        Ok(this)
    }

    /// Returns the value of the Number.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::BitRange;
    /// let v: BitRange<8, 16> = BitRange::new_number(123u8, None).unwrap();
    /// assert_eq!(v.number::<u8>(), 123);
    /// ```
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
    /// # use varint_core::BitRange;
    /// let mut v: BitRange<8, 16> = BitRange::default();
    /// v.set_number(123u8, None).unwrap();
    /// assert_eq!(v, 123);
    /// ```
    ///
    /// # Error
    ///
    /// ```
    /// # use varint_core::BitRange;
    /// let mut v: BitRange<0, 2> = BitRange::default();
    /// assert!(v.set_number(20u8, None).is_err());
    /// ```
    pub fn set_number<U>(&mut self, v: U, n: Option<usize>) -> Result<&mut Self, BitRangeError<U>>
    where
        U: Unsigned,
    {
        let num_bits = super::num_bits(v);
        let len = match n {
            Some(bits) => {
                if bits < MIN {
                    MIN
                } else {
                    bits
                }
            }
            None => num_bits,
        };
        // TODO error doesn't fit
        // snafu::ensure!(num_bits >= len, )

        // ensure v fits
        snafu::ensure!(
            len <= MAX,
            InvalidCapacitySnafu {
                value: v,
                needs: len,
                cap: MAX,
            }
        );

        let len = if len < MIN { MIN } else { len };

        self.data
            .set_number(v, Some(len))
            .context(BitStoreNumberSnafu)?;

        Ok(self)
    }

    pub fn new_bytes(buf: &[u8], n: usize) -> Result<Self, BitRangeBytesError> {
        let mut this = Self::default();
        this.set_bytes(buf, n)?;
        Ok(this)
    }

    pub fn bytes(&self) -> Bytes {
        self.data.bits()
    }

    pub fn set_bytes(&mut self, buf: &[u8], n: usize) -> Result<&mut Self, BitRangeBytesError> {
        self.data.set_bits(buf, n).context(BitStoreBytesSnafu)?;
        Ok(self)
    }
}

impl<const MIN: usize, const MAX: usize> Debug for BitRange<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitRange")
            .field("min_bits", &MIN)
            .field("max_bits", &MAX)
            .field("inner", &self.data)
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> VarInt for BitRange<MIN, MAX> {
    type Error = DecodeError;
    fn decode<R>(reader: &mut R, length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        let Some(length) = length else {
            return Err(DecodeError::MissingLength2);
        };

        snafu::ensure!(
            length >= MIN && length <= MAX,
            InvalidBitLength2Snafu {
                got: length,
                min: MIN,
                max: MAX
            }
        );

        let buf = reader.read_bits(length).context(ReadSnafu)?;

        let mut this = Self::default();
        this.set_bytes(&buf, length).context(TodoSnafu)?;

        Ok((this, length))
    }

    fn encode<W>(&self, writer: &mut W, length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        // TODO Snafu context instead
        let Some(length) = length else {
            return Err(DecodeError::MissingLength2);
        };
        let bits = self.data.bits();

        writer.write_bits(length, &bits);

        Ok(length)
    }

    fn len_bits(&self) -> usize {
        self.data.len()
    }

    fn length_required() -> bool {
        // length is variable and needs to be provided
        true
    }
}

#[derive(Debug, Snafu, PartialEq)]
pub enum BitRangeError<U>
where
    U: Unsigned,
{
    /// number needs more bits than available
    #[snafu(display("value >{value}< needs >{needs}< bits space, but only >{cap}< bits can fit"))]
    InvalidCapacity {
        value: U,
        needs: usize,
        cap: usize,
    },
    /// decoding a [BitRange] requires a specified length in bits,
    /// usually provided by a preceding number specifying the
    /// length
    #[snafu(display("decoding requires a length"))]
    MissingLength,
    ReaderError {
        source: ReaderError,
    },
    BitStoreNumberError {
        source: bitstore::Error,
    },
    ByteError {
        source: BitRangeBytesError,
    },
}

impl<U> BitRangeError<U>
where
    U: Unsigned,
{
    pub fn cast(self) -> BitRangeError<u128> {
        match self {
            Self::BitStoreNumberError { source } => BitRangeError::BitStoreNumberError { source },
            Self::ByteError { source } => BitRangeError::ByteError { source },
            Self::InvalidCapacity { value, needs, cap } => BitRangeError::InvalidCapacity {
                value: value.as_u128(),
                needs,
                cap,
            },
            Self::MissingLength => BitRangeError::MissingLength,
            Self::ReaderError { source } => BitRangeError::ReaderError { source },
        }
    }
}

#[derive(Debug, Snafu, PartialEq)]
pub enum BitRangeBytesError {
    BitStoreBytesError {
        source: bitstore::Error,
    },
    /// length is not between the required bounds
    #[snafu(display("invalid length, got >{got}<, but need between >{min}< and >{max}<"))]
    InvalidBitLength {
        got: usize,
        min: usize,
        max: usize,
    },
}

#[derive(Debug, Snafu, PartialEq)]
pub enum DecodeError {
    /// decoding a [BitRange] requires a specified length in bits,
    /// usually provided by a preceding number specifying the
    /// length
    #[snafu(display("decoding requires a length"))]
    MissingLength2,
    /// length is not between the required bounds
    #[snafu(display("invalid length, got >{got}<, but need between >{min}< and >{max}<"))]
    InvalidBitLength2 {
        got: usize,
        min: usize,
        max: usize,
    },
    ReadError {
        source: ReaderError,
    },
    Todo {
        source: BitRangeBytesError,
    },
}

#[derive(Debug, Snafu, PartialEq)]
pub enum ConversionError<I>
where
    I: Integral,
{
    /// Error when trying to create a [Number] from a negative integer
    #[snafu(display("VarInt Number cannot be negative, trying >{value}<"))]
    IsNegative { value: I },
    /// Error when [BitRange::new] fails
    #[snafu(display("failed to create a BitRange from >{value}<"))]
    Invalid {
        value: I,
        source: BitRangeError<u128>,
    },
    /// Error when trying to cast a Number into a too small type
    #[snafu(display("BitRange >{value}< does not fit into the given type, max value: >{max}<"))]
    UnFit { value: u128, max: I },
}

macro_rules! impl_partial_eq {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const MIN: usize, const MAX: usize> PartialEq<$prim> for BitRange<MIN, MAX> {
                fn eq(&self, other: &$prim) -> bool {
                    if *other < (0 as $prim) { return false; }
                    // now that it is verified that other is
                    // not negative or larger than allowed
                    // it is permissible to case both to u64
                    *other as u128 == self.number::<u128>()
                }
            }
            impl<const MIN: usize, const MAX: usize> PartialEq<BitRange<MIN, MAX>> for $prim {
                fn eq(&self, other: &BitRange<MIN, MAX>) -> bool {
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
            impl<const MIN: usize, const MAX: usize> PartialOrd<$prim> for BitRange<MIN, MAX> {
                fn partial_cmp(&self, other: &$prim) -> Option<std::cmp::Ordering> {
                    if *other < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // other is positive -> casting to u128 is fine
                    let other = *other as u128;
                    if self.number::<u128>() > other {
                        Some(std::cmp::Ordering::Greater)
                    } else if self.number::<u128>() < other {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }
            }
            impl<const MIN: usize, const MAX: usize> PartialOrd<BitRange<MIN, MAX>> for $prim {
                fn partial_cmp(&self, other: &BitRange<MIN, MAX>) -> Option<std::cmp::Ordering> {
                    if *self < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // self is positive -> casting to u128 is fine
                    if (*self as u128) > other.number::<u128>() {
                        Some(std::cmp::Ordering::Greater)
                    } else if (*self as u128) < other.number::<u128>() {
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

macro_rules! impl_try_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const MIN: usize, const MAX: usize> TryFrom<$prim> for BitRange<MIN, MAX> {
                type Error = ConversionError<$prim>;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(value >= (0 as $prim), IsNegativeSnafu { value });
                    // value is verified to be positive => casting
                    // to u128 is permissible
                    Self::new_number(value as u128, None).context(InvalidSnafu { value })
                }
            }
            impl<const MIN: usize, const MAX: usize> TryFrom<BitRange<MIN, MAX>> for $prim {
                type Error = ConversionError<$prim>;
                fn try_from(value: BitRange<MIN, MAX>) -> Result<Self, Self::Error> {
                    snafu::ensure!(value <= <$prim>::MAX, UnFitSnafu { value: value.number::<u128>(), max: <$prim>::MAX });

                    Ok(value.number::<u128>() as $prim)
                }
            }
        )+
    };
}
impl_try_from!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl PartialEq<&str> for BitRange {
    fn eq(&self, other: &&str) -> bool {
        self.bytes() == other.as_bytes()
    }
}

impl PartialEq<String> for BitRange {
    fn eq(&self, other: &String) -> bool {
        self.bytes() == other.as_bytes()
    }
}

impl PartialEq<&[u8]> for BitRange {
    fn eq(&self, other: &&[u8]) -> bool {
        self.bytes() == other
    }
}

// TODO From strings and bytes
// TODO TryFrom for BitRange with generics
impl From<&str> for BitRange {
    fn from(value: &str) -> Self {
        Self::new_bytes(value.as_bytes(), value.len() * 8).unwrap()
    }
}

impl From<String> for BitRange {
    fn from(value: String) -> Self {
        Self::new_bytes(value.as_bytes(), value.len() * 8).unwrap()
    }
}

impl From<&[u8]> for BitRange {
    fn from(value: &[u8]) -> Self {
        Self::new_bytes(value, value.as_ref().len() * 8).unwrap()
    }
}

impl<const N: usize> From<[u8; N]> for BitRange {
    fn from(value: [u8; N]) -> Self {
        Self::new_bytes(&value, N * 8).unwrap()
    }
}

impl<const N: usize> From<&[u8; N]> for BitRange {
    fn from(value: &[u8; N]) -> Self {
        Self::new_bytes(value, N * 8).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Number, ReferenceReader, ReferenceWriter, Writer};

    use super::*;

    #[test]
    fn from_buf_test() {
        const BUF: &[u8] = &[1, 2, 3];

        let valid = BitRange::from(BUF);
        assert_eq!(valid, BUF);
    }

    #[test]
    fn from_str_test() {
        const STR_SLICE: &str = "message";
        let static_str: &'static str = "data";
        let owned_string = "my stuff".to_owned();

        let valid = BitRange::from(STR_SLICE);
        assert_eq!(valid, STR_SLICE);

        let valid = BitRange::from(static_str);
        assert_eq!(valid, static_str);

        let valid = BitRange::from(owned_string.clone());
        assert_eq!(valid, owned_string);
    }

    #[test]
    fn set_number_test() {
        let mut base: BitRange<8, 16> = BitRange::default();

        base.set_number(8u8, None).unwrap();
        assert_eq!(base.data.number::<u16>(), 8);

        let invalid = base.set_number(u32::MAX, None).unwrap_err();
        assert_eq!(
            invalid,
            BitRangeError::InvalidCapacity {
                value: u32::MAX,
                needs: 32,
                cap: 16
            }
        );
    }

    #[test]
    fn new_number_test() {
        let valid: BitRange<8, 16> = BitRange::new_number(123u16, None).unwrap();
        assert_eq!(valid.data.number::<u16>(), 123);

        let invalid = BitRange::<8, 16>::new_number(u32::MAX, None).unwrap_err();
        assert_eq!(
            invalid,
            BitRangeError::InvalidCapacity {
                value: u32::MAX,
                needs: 32,
                cap: 16
            }
        );
    }

    #[test]
    fn eq_test() {
        assert_eq!(BitRange::<0>::new_number(16u8, None).unwrap(), 16u32);
        assert_eq!(BitRange::<0>::new_number(100u16, None).unwrap(), 100i32);
        assert_eq!(BitRange::<0>::default(), 0);

        assert_ne!(BitRange::<8, 16>::new_number(100u128, None).unwrap(), -5);
        assert_ne!(BitRange::<8, 16>::default(), 100);
        assert_ne!(BitRange::<8, 16>::new_number(100u128, None).unwrap(), 1);
    }

    #[test]
    fn ord_test() {
        let num: BitRange<8, 16> = BitRange::new_number(123u8, None).unwrap();
        assert!(num > 50u8);
        assert!(50u64 < num);
        assert!(num > 13i8);
        assert!(num < 250u16);
        assert!(250u16 > num);
        assert!(num == 123u64);
        assert!(123u8 == num);
    }

    #[test]
    fn try_from_test() {
        let valid: BitRange<8, 16> = BitRange::try_from(5_123i16).unwrap();
        assert_eq!(valid, 5_123u16);

        let invalid = BitRange::<0, 2>::try_from(100).unwrap_err();
        assert_eq!(
            invalid,
            ConversionError::Invalid {
                value: 100,
                source: BitRangeError::InvalidCapacity {
                    value: 100,
                    needs: 7,
                    cap: 2
                }
            }
        );

        let invalid = BitRange::<8, 16>::try_from(-5).unwrap_err();
        assert_eq!(invalid, ConversionError::IsNegative { value: -5 });

        assert_eq!(u16::try_from(valid.clone()).unwrap(), 5_123);
        assert_eq!(
            u8::try_from(valid).unwrap_err(),
            ConversionError::UnFit {
                value: 5_123,
                max: u8::MAX
            }
        );
    }

    #[test]
    fn new_bytes_test() {
        let valid = BitRange::<8, 16>::new_bytes(&[0b1100_0011, 0b0011_0000], 12).unwrap();
        assert_eq!(*valid.bytes(), [0b1100_0011, 0b0011_0000]);
        // TODO
    }

    #[test]
    fn bytes_test() {
        let num = BitRange::<8, 64>::new_number(u32::MAX as u64 + 1, None).unwrap();

        assert_eq!(*num.bytes(), [128, 0, 0, 0, 0]);
        assert_eq!(num, u32::MAX as u64 + 1)
    }

    #[test]
    fn set_bytes_test() {
        let mut num = BitRange::<8, 16>::default();
        num.set_bytes([1u8, 0].as_slice(), 9).unwrap();

        assert_eq!(num, 2);
        assert_eq!(*num.bytes(), [1, 0]);
    }

    const BUFFER: &[u8] = &[
        // x(i) = 20
        0b0001_0100,
        // 20 bits = 0b0000 0b0000_0000 0b0000_1000 = 8
        0b0000_0000,
        0b0000_0000,
        0b1000_0000, // final 12 bits = 0b0000 0b0001_0000 = 16
        0b0001_0000,
    ];

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUFFER);
        let mut length = BUFFER.len() * 8;

        let (len, bits) = Number::decode(&mut reader, None).unwrap();
        assert_eq!(bits, 8);
        length -= bits;

        let (valid, bits) = BitRange::<8, 32>::decode(&mut reader, Some(len.number())).unwrap();
        assert_eq!(bits, len);
        assert_eq!(valid, 8);
        assert_eq!(*valid.bytes(), [0b0000_0000, 0b0000_0000, 0b1000_0000]);
        length -= bits;

        let (valid, bits) = BitRange::<0>::decode(&mut reader, Some(length)).unwrap();
        assert_eq!(bits, 12);
        assert_eq!(valid, 16);
        assert_eq!(*valid.bytes(), [0b0000_0001, 0b0000]);
        length -= bits;

        assert!(length == 0);
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();
        let mut length = BUFFER.len() * 8;

        let len = Number::new(20u8);
        let bits = len.encode(&mut writer, None).unwrap();
        assert_eq!(bits, 8);
        length -= bits;

        let num = BitRange::<8, 32>::new_number(8u8, Some(len.number())).unwrap();
        let bits = num.encode(&mut writer, Some(len.number())).unwrap();
        assert_eq!(bits, 20);
        length -= bits;

        let num = BitRange::<0>::new_number(16u8, Some(length)).unwrap();
        let bits = num.encode(&mut writer, Some(length)).unwrap();
        assert_eq!(bits, 12);
        length -= bits;

        assert!(length == 0);

        assert_eq!(writer.finish().unwrap(), BUFFER);
    }
}
