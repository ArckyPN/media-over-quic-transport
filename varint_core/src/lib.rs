pub(crate) mod bitstore;
mod io;
pub mod types;

pub use io::{
    reader::{Reader, ReaderError, ReferenceReader},
    writer::{ReferenceWriter, Writer, WriterError},
};
use snafu::{ResultExt, Snafu};
pub use types::{BitNumber, BitNumberError, BitRange, BitRangeError, Number, NumberError};

#[cfg(feature = "moq")]
pub use types::{BinaryData, BinaryDataError, Tuple, TupleError};

pub trait VarInt {
    /// The error type.
    type Error;

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
        R: Reader,
        Self: std::marker::Sized;

    /// Returns the number of bits required to represent
    /// the data as VarInt.
    fn len_bits(&self) -> usize;

    /// Indicates whether this types requires the `length`
    /// argument dor `encode` and `decode`.
    ///
    /// This is connected to the [BitRange] aka. `x!(..)`
    /// type, which has a variable length and requires a
    /// specified length.
    ///
    /// On a struct with `x!(..)` as final field, this will
    /// signal that the remaining buffer will be used.
    fn length_required() -> bool;
}

// TODO get rid of all unwraps
// TODO impl VarInt for primitive types, which make sense
// TODO rework all Error types, every fn should have its own Error type, errors can be reused if fns share the same

#[derive(Debug, Snafu, PartialEq, Clone)]
pub struct NumberStringError {}

#[derive(Debug, Snafu, PartialEq, Clone)]
pub enum StringError {
    #[snafu(display("Negative Numbers are not supported"))]
    IsNegative,
    #[snafu(display("Failed to parse >{input}< as number"))]
    NumberError {
        input: String,
        source: std::num::ParseIntError,
    },
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
    N::from_str_radix(src, radix).context(NumberSnafu {
        input: s.to_string(),
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
                source: "not a number".parse::<u8>().unwrap_err()
            })
        );
    }
}
