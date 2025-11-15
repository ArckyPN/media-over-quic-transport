pub(crate) mod bitstore;
#[cfg(feature = "moq")]
pub mod external_impls;
mod io;
pub mod types;

#[cfg(feature = "moq")]
use std::fmt::Display;

use snafu::Snafu;
pub use {
    io::{
        reader::{Reader, ReaderError, ReferenceReader},
        writer::{ReferenceWriter, Writer, WriterError},
    },
    types::{BitNumber, BitNumberError, BitRange, BitRangeError, Number, NumberError},
};

#[cfg(feature = "moq")]
pub use types::{BinaryData, BinaryDataError, Tuple, TupleError};

pub trait VarInt
where
    Self: std::marker::Sized,
{
    /// The error type.
    type Error: std::fmt::Debug;

    /// Encode the data for transmission.
    ///
    /// The `length` (in bits) is needed by some type for
    /// correct encoding. Usually, those type are
    /// preceded by its length. Types that don't
    /// need a specified length ignore this length.
    ///
    /// On success the number of bits encoded is returned.
    fn encode<W>(&self, writer: &mut W, length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: Writer;

    /// Decode received data.
    ///
    /// The `length` (in bits) is needed by some types for
    /// correct decoding. Usually, those types are
    /// preceded by its length. Types that don't
    /// need a specified length ignore this length.
    ///
    /// The length can also be used to validate that
    /// a message is fully decoded.
    ///
    /// On success the decoded data and the number of
    /// decoded bits is returned.
    fn decode<R>(reader: &mut R, length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: Reader;

    /// Returns the number of bits required to represent
    /// the data as VarInt.
    ///
    /// Return an Error if the number of bits cannot be
    /// determined.
    fn len_bits(&self) -> Result<usize, Self::Error>;

    /// Indicates whether this types requires the `length`
    /// argument for `encode` and `decode`.
    ///
    /// This is connected to the [BitRange] aka. `x!(..)`
    /// type, which has a variable length and requires a
    /// specified length.
    ///
    /// On a struct with `x!(..)` as final field, this will
    /// signal that the remaining buffer will be used.
    fn length_required() -> bool;
}

// TODO add funty::Unsigned bound
pub trait VarIntNumber: VarInt
where
    Self: Default,
{
    /// Returns the inner buffer as unsigned
    /// number.
    fn number<U>(&self) -> U
    where
        U: funty::Unsigned;

    /// Constructs a new Number from any unsigned
    /// Number `v`, using the `n` MSBs.
    ///
    /// Use all bits is `n` is None.
    fn new_number<U>(v: U, n: Option<usize>) -> Result<Self, Self::Error>
    where
        U: funty::Unsigned,
    {
        let mut this = Self::default();
        this.set_number(v, n)?;
        Ok(this)
    }

    /// Overrides the inner buffer with a new unsigned
    /// Number `v`, using the `n` MSBs.
    ///
    /// Use all bits is `n` is None.
    fn set_number<U>(&mut self, v: U, n: Option<usize>) -> Result<&mut Self, Self::Error>
    where
        U: funty::Unsigned;
}

pub trait VarIntBytes: VarInt {
    /// Returns the inner buffer as raw Bytes.
    fn bytes(&self) -> bytes::Bytes;

    /// Constructs a new Bytes value with `n` bits
    /// from `buf`fer, starting at the MSB.
    ///
    /// If `n` is None, the entire `buf`fer will be used.
    fn new_bytes(buf: &[u8], n: Option<usize>) -> Result<Self, Self::Error>;

    /// Overrides the inner buffer with `n` bits
    /// from `buf`fer, starting at the MSB.
    ///
    /// If `n` is None, the entire `buf`fer will be used.
    fn set_bytes(&mut self, buf: &[u8], n: Option<usize>) -> Result<&mut Self, Self::Error>;
}

/// A MOQT Parameter is part of he Key-Value-Pair Structure.
///
/// A Parameter is defined as a [VarInt](Number) `x(i)` Key.
///
/// This Key defines the encoding of the associated
/// Value:
///
///  - number value of Key is even: Value is a [VarInt](Number)
///    number
///  - number value of Key if odd: Value is a sequence of Bytes
///    as [BitRange]
///    - next bytes is another [VarInt](Number) number specifying
///      the number of bytes
///    - the remainder are that number of bytes
///
/// ---
///
/// The Parameter `trait` is a super trait of [TryFrom] for [KeyValuePair](external_impls::KeyValuePair)
/// and the method [to_kvp](Parameter::to_kvp), which convert the Parameter back
/// and forth from a [KeyValuePair](external_impls::KeyValuePair).
///
/// A [KeyValuePair](external_impls::KeyValuePair) is decoded and then converted the
/// actual parameter key and value can be extracted from that. Encoding does this
/// in reverse.
#[cfg(feature = "moq")]
pub trait Parameter: TryFrom<external_impls::KeyValuePair> {
    /// The Parameter Error
    type PError: Display;

    /// Converts a Parameter back to a Key-Value-Pair for encoding.
    fn to_kvp(&self, key: Number) -> Result<external_impls::KeyValuePair, Self::PError>;
}

#[derive(Debug, Snafu, PartialEq, Clone)]
pub struct NumberStringError {}

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum StringError {
    #[snafu(display("Negative Numbers are not supported"))]
    IsNegative,
    #[snafu(display("Failed to parse {input} as number"))]
    NumberError { input: String, cause: String },
}

pub(crate) fn number_from_str<N: funty::Integral>(s: &str) -> Result<N, StringError> {
    snafu::ensure!(!s.starts_with("-"), IsNegativeSnafu);
    let (src, radix) = if let Some(src) = s.strip_prefix("0b") {
        // binary
        (src, 2)
    } else if let Some(src) = s.strip_prefix("0o") {
        // octal
        (src, 8)
    } else if let Some(src) = s.strip_prefix("0x") {
        // hexadecimal
        (src, 16)
    } else {
        // decimal
        (s, 10)
    };
    N::from_str_radix(src, radix).map_err(|err| StringError::NumberError {
        input: s.to_string(),
        cause: err.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn number_from_str_test() {
        let valid_dec = number_from_str::<u16>("15");
        assert_eq!(valid_dec, Ok(15));

        let valid_bin = number_from_str::<u8>("0b11001");
        assert_eq!(valid_bin, Ok(25));

        let valid_oct = number_from_str::<i64>("0o072");
        assert_eq!(valid_oct, Ok(58));

        let valid_hex = number_from_str::<u128>("0xAF");
        assert_eq!(valid_hex, Ok(175));

        let invalid = number_from_str::<i8>("-0x1");
        assert_eq!(invalid, Err(StringError::IsNegative));

        let invalid = number_from_str::<u8>("not a number");
        assert_eq!(
            invalid,
            Err(StringError::NumberError {
                input: "not a number".to_owned(),
                cause: "not a number".parse::<u8>().unwrap_err().to_string()
            })
        );
    }
}
