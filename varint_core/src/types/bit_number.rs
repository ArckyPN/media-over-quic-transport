use std::fmt::{Debug, Display};

use funty::{Integral, Unsigned};
use snafu::{ResultExt, Snafu};

use crate::{
    VarInt,
    bitstore::{self, BitStore},
    io::reader::ReaderError,
};

/// This type represents several types of
/// the QUIC RFC:
///
/// - `x (N)` -> `BitNumber<N>` a Number
///   represented by `N` bits
/// - `x (N) = C` -> `BitNumber<N, C, C>`
///   a Number represented by `N` Bits
///   with the const value `C`
/// - `x (N) = C..D` -> `BitNumber<N, C, D>`
///   a Number represented by `N` Bits
///   with a value between `C` and `D`
///   (inclusive)
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct BitNumber<const N: usize, const MIN: u128 = 0, const MAX: u128 = { u128::MAX }> {
    data: BitStore<N, N>,
}

impl<const N: usize, const MIN: u128, const MAX: u128> BitNumber<N, MIN, MAX> {
    /// Constructs a new Number.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::BitNumber;
    /// let v: BitNumber<8> = BitNumber::new(123u8).unwrap();
    /// assert_eq!(v, 123);
    /// ```
    ///
    /// # Error
    ///
    /// ```
    /// # use varint_core::{BitNumber, BitNumberError};
    /// let v: Result<BitNumber<2>, BitNumberError<u8>> = BitNumber::new(123u8);
    /// assert!(v.is_err());
    /// ```
    pub fn new<U>(v: U) -> Result<Self, BitNumberError<U>>
    where
        U: Unsigned, // TODO can probably this Integral and add a verification if it is positive
    {
        let mut this = Self::default();
        this.set_number(v)?;
        Ok(this)
    }

    /// Returns the value of the Number.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::BitNumber;
    /// let v: BitNumber<8> = BitNumber::new(123u16).unwrap();
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
    /// # use varint_core::BitNumber;
    /// let mut v: BitNumber<16> = BitNumber::default();
    /// v.set_number(123u16).unwrap();
    /// assert_eq!(v, 123);
    /// ```
    ///
    /// # Error
    ///
    /// ```
    /// # use varint_core::BitNumber;
    /// let mut v: BitNumber<8, 4, 16> = BitNumber::default();
    /// assert!(v.set_number(20u8).is_err());
    /// ```
    pub fn set_number<U>(&mut self, v: U) -> Result<&mut Self, BitNumberError<U>>
    where
        U: Unsigned,
    {
        let len = super::num_bits(v);

        // ensure v fits
        snafu::ensure!(
            len <= N,
            InvalidCapacitySnafu {
                value: v,
                needs: len,
                cap: N
            }
        );
        snafu::ensure!(
            v.as_u128() >= MIN && v.as_u128() <= MAX,
            OutOfRangeSnafu {
                value: v,
                min: MIN,
                max: MAX
            }
        );

        self.data.set_number(v, Some(N)).context(BitStoreSnafu)?;

        Ok(self)
    }
}

impl<const N: usize, const MIN: u128, const MAX: u128> Display for BitNumber<N, MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.number::<u128>().to_string())
    }
}

impl<const N: usize, const MIN: u128, const MAX: u128> Debug for BitNumber<N, MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitNumber")
            .field("value", &self.number::<u128>())
            .field("num_bits", &N)
            .field("min_value", &MIN)
            .field("max_value", &MAX)
            .field("inner", &self.data)
            .finish()
    }
}

impl<const N: usize, const MIN: u128, const MAX: u128> VarInt for BitNumber<N, MIN, MAX> {
    type Error = BitNumberError<u128>;
    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        let buf = reader.read_bits(N).context(ReaderSnafu)?.to_vec();

        let mut this = Self::default();
        this.data.set_bits(&buf, N).context(BitStoreSnafu)?;

        let num = this.number::<u128>();
        snafu::ensure!(
            num >= MIN && num <= MAX,
            OutOfRangeSnafu {
                value: num,
                min: MIN,
                max: MAX
            }
        );

        Ok((this, N))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let buf = self.data.bits();

        writer.write_bits(N, &buf);

        Ok(N)
    }

    fn len_bits(&self) -> usize {
        N
    }

    fn length_required() -> bool {
        // length is provided by the generic bound N
        false
    }
}

#[derive(Debug, Snafu, PartialEq)]
pub enum BitNumberError<U>
where
    U: Unsigned,
{
    ReaderError {
        source: ReaderError,
    },
    BitStoreError {
        source: bitstore::Error,
    },
    /// number is outside of the specified range
    #[snafu(display("value >{value}< doesn't fit into specified range [{min}; {max}]"))]
    OutOfRange {
        value: U,
        min: u128,
        max: u128,
    },
    /// number needs more bits than available
    #[snafu(display("value >{value}< needs >{needs}< bits space, but only >{cap}< bits can fit"))]
    InvalidCapacity {
        value: U,
        needs: usize,
        cap: usize,
    },
}

impl<U> BitNumberError<U>
where
    U: Unsigned,
{
    pub fn cast(self) -> BitNumberError<u128> {
        match self {
            Self::BitStoreError { source } => BitNumberError::BitStoreError { source },
            Self::InvalidCapacity { value, needs, cap } => BitNumberError::InvalidCapacity {
                value: value.as_u128(),
                needs,
                cap,
            },
            Self::OutOfRange { value, min, max } => BitNumberError::OutOfRange {
                value: value.as_u128(),
                min,
                max,
            },
            Self::ReaderError { source } => BitNumberError::ReaderError { source },
        }
    }
}

#[derive(Debug, Snafu, PartialEq)]
pub enum ConversionError<I>
where
    I: Integral,
{
    /// Error when trying to create a [BitNumber] from a negative integer
    #[snafu(display("VarInt Number cannot be negative, trying >{value}<"))]
    IsNegative { value: I },
    /// Error when [BitNumber::new] fails
    #[snafu(display("failed to create a BitNumber from >{value}<"))]
    Invalid {
        value: I,
        source: BitNumberError<u128>,
    },
    /// Error when trying to cast a Number into a too small type
    #[snafu(display("BitNumber >{value}< does not fit into the given type, max value: >{max}<"))]
    UnFit { value: u128, max: I },
}

macro_rules! impl_partial_eq {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialEq<$prim> for BitNumber<N, MIN, MAX> {
                fn eq(&self, other: &$prim) -> bool {
                    if *other < (0 as $prim) { return false; }
                    // now that it is verified that other is
                    // not negative or larger than allowed
                    // it is permissible to case both to u64
                    *other as u128 == self.number::<u128>()
                }
            }
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialEq<BitNumber<N, MIN, MAX>> for $prim {
                fn eq(&self, other: &BitNumber<N, MIN, MAX>) -> bool {
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
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialOrd<$prim> for BitNumber<N, MIN, MAX> {
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
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialOrd<BitNumber<N, MIN, MAX>> for $prim {
                fn partial_cmp(&self, other: &BitNumber<N, MIN, MAX>) -> Option<std::cmp::Ordering> {
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
            impl<const N: usize, const MIN: u128, const MAX: u128> TryFrom<$prim> for BitNumber<N, MIN, MAX> {
                type Error = ConversionError<$prim>;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(value >= (0 as $prim), IsNegativeSnafu { value });
                    // value is verified to be positive => casting
                    // to u128 is permissible
                    Self::new(value as u128).context(InvalidSnafu { value })
                }
            }
            impl<const N: usize, const MIN: u128, const MAX: u128> TryFrom<BitNumber<N, MIN, MAX>> for $prim {
                type Error = ConversionError<$prim>;
                fn try_from(value: BitNumber<N, MIN, MAX>) -> Result<Self, Self::Error> {
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

#[cfg(test)]
mod tests {
    use crate::{ReferenceReader, ReferenceWriter, Writer, io::writer::WriterError};

    use super::*;

    #[test]
    fn set_number_test() {
        let mut num = BitNumber::<16>::default();
        let valid = num.set_number(15u8);
        assert!(valid.is_ok());
        assert_eq!(num.data.number::<u16>(), 15);

        let invalid = num.set_number(u32::MAX);
        assert_eq!(
            invalid,
            Err(BitNumberError::InvalidCapacity {
                value: u32::MAX,
                needs: 32,
                cap: 16
            })
        );

        let mut num = BitNumber::<8, 5>::default();
        let valid = num.set_number(100u8);
        assert!(valid.is_ok());
        assert_eq!(num.data.number::<u16>(), 100);

        let invalid = num.set_number(3u8);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 3u8,
                min: 5,
                max: u128::MAX
            })
        );

        let mut num = BitNumber::<8, 0, 20>::default();
        let valid = num.set_number(20u8);
        assert!(valid.is_ok());
        assert_eq!(num.data.number::<u16>(), 20);

        let invalid = num.set_number(21u8);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 21u8,
                min: 0,
                max: 20
            })
        );
    }

    #[test]
    fn new_test() {
        let num = BitNumber::<16>::new(15u8);
        assert_eq!(num.map(|n| n.data.number::<u16>()), Ok(15));

        let invalid = BitNumber::<16>::new(u32::MAX);
        assert_eq!(
            invalid,
            Err(BitNumberError::InvalidCapacity {
                value: u32::MAX,
                needs: 32,
                cap: 16
            })
        );

        let num = BitNumber::<8, 5>::new(100u8);
        assert_eq!(num.map(|n| n.data.number::<u16>()), Ok(100));

        let invalid = BitNumber::<8, 5>::new(3u8);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 3u8,
                min: 5,
                max: u128::MAX
            })
        );

        let num = BitNumber::<8, 0, 20>::new(20u8);
        assert_eq!(num.map(|n| n.data.number::<u16>()), Ok(20));

        let invalid = BitNumber::<8, 0, 20>::new(21u8);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 21u8,
                min: 0,
                max: 20
            })
        );
    }

    #[test]
    fn eq_test() {
        assert_eq!(BitNumber::<8>::new(100u8).unwrap(), 100u16);
        assert_eq!(BitNumber::<16, 10>::new(10u16).unwrap(), 10i16);
        assert_eq!(BitNumber::<8, 0, 100>::new(100u8).unwrap(), 100u16);

        assert_ne!(BitNumber::<8>::new(100u8).unwrap(), 101i128);
        assert_ne!(BitNumber::<16, 10>::new(10u16).unwrap(), -5);
        assert_ne!(BitNumber::<32, 0, 1>::new(1u128).unwrap(), 100u16);
    }

    #[test]
    fn ord_test() {
        let num: BitNumber<32, 5, 1_000> = BitNumber::new(123u8).unwrap();
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
        let valid: BitNumber<16> = BitNumber::try_from(5_123u16).unwrap();
        assert_eq!(valid, 5_123u64);

        let valid: BitNumber<16, 5> = BitNumber::try_from(5i8).unwrap();
        assert_eq!(valid, 5u8);

        let invalid = BitNumber::<8, 0, 10>::try_from(11);
        assert_eq!(
            invalid,
            Err(ConversionError::Invalid {
                value: 11,
                source: BitNumberError::OutOfRange {
                    value: 11,
                    min: 0,
                    max: 10
                }
            })
        );

        let invalid = BitNumber::<8>::try_from(-1);
        assert_eq!(invalid, Err(ConversionError::IsNegative { value: -1 }));

        let invalid = BitNumber::<1>::try_from(2u128);
        assert_eq!(
            invalid,
            Err(ConversionError::Invalid {
                value: 2,
                source: BitNumberError::InvalidCapacity {
                    value: 2,
                    needs: 2,
                    cap: 1
                }
            })
        );

        let num: BitNumber<16> = BitNumber::try_from(5_123u16).unwrap();
        assert_eq!(u16::try_from(num.clone()).unwrap(), 5_123);
        assert_eq!(
            u8::try_from(num).unwrap_err(),
            ConversionError::UnFit {
                value: 5_123,
                max: u8::MAX
            }
        );
    }

    const BUFFER: &[u8] = &[
        0b0000_0000,
        0b0100_0000,
        0b0001_1111,
        0b0011_1000,
        0b0000_0000,
        0b0111_1011,
    ];

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUFFER);

        let (num, bits) = BitNumber::<20>::decode(&mut reader, None).unwrap();
        assert_eq!(bits, 20);
        assert_eq!(num, 1025);

        let (num, bits) = BitNumber::<4>::decode(&mut reader, None).unwrap();
        assert_eq!(bits, 4);
        assert_eq!(num, 15);

        let (num, bits) = BitNumber::<5>::decode(&mut reader, None).unwrap();
        assert_eq!(bits, 5);
        assert_eq!(num, 7);

        let (num, bits): (BitNumber<19>, _) = BitNumber::decode(&mut reader, None).unwrap();
        assert_eq!(bits, 19);
        assert_eq!(num, 123);
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let num = BitNumber::<20>::new(1025u16).unwrap();
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(20));

        let num = BitNumber::<4>::new(15u8).unwrap();
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(4));

        let num: BitNumber<5> = BitNumber::new(7u32).unwrap();
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(5));

        let num: BitNumber<19> = BitNumber::new(123u64).unwrap();
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(19));

        let valid = writer.finish().unwrap();
        assert_eq!(valid, BUFFER);

        let mut writer = ReferenceWriter::new();
        let valid = BitNumber::<5>::new(13u8).unwrap().encode(&mut writer, None);
        assert_eq!(valid, Ok(5));

        let invalid = writer.finish();
        assert_eq!(invalid, Err(WriterError::LoosePartialByte));
    }
}
