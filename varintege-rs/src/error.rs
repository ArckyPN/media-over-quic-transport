use funty::{Integral, Unsigned};
use snafu::Snafu;
use varint_core::{
    BinaryDataError, BitNumberError, NumberError, TupleError,
    types::{bit_number, bit_range, number},
};

#[derive(Debug, Snafu, PartialEq)]
#[snafu(visibility(pub), module(ctx))]
pub enum Error {
    #[snafu(display("BinaryData: {source}"))]
    BinaryData { source: BinaryDataError },
    #[snafu(display("BitNumber: {source}"))]
    BitNumber { source: BitNumberError<u128> },
    #[snafu(display("BitRange: {source}"))]
    BitRange { source: bit_range::DecodeError },
    #[snafu(display("Conversion: {cause}"))]
    ConversionError { cause: String },
    #[snafu(display("expected to decode {expected} bits, but read {got}"))]
    LengthMismatch { expected: usize, got: usize },
    #[snafu(display("a length was expected by the type"))]
    MissingLength,
    #[snafu(display("Number: {source}"))]
    Number { source: NumberError<u128> },
    #[snafu(display("Tuple: {source}"))]
    Tuple { source: TupleError },
    #[snafu(display("Unknown Value: got {value}"))]
    UnknownValue { value: u128 },
}

impl From<BinaryDataError> for Error {
    fn from(value: BinaryDataError) -> Self {
        Self::BinaryData { source: value }
    }
}

impl<U> From<BitNumberError<U>> for Error
where
    U: Unsigned,
{
    fn from(value: BitNumberError<U>) -> Self {
        Self::BitNumber {
            source: value.cast(),
        }
    }
}

impl From<bit_range::DecodeError> for Error {
    fn from(value: bit_range::DecodeError) -> Self {
        Self::BitRange { source: value }
    }
}

impl<I> From<bit_number::ConversionError<I>> for Error
where
    I: Integral,
{
    fn from(value: bit_number::ConversionError<I>) -> Self {
        Self::ConversionError {
            cause: value.to_string(),
        }
    }
}

impl<I> From<bit_range::ConversionError<I>> for Error
where
    I: Integral,
{
    fn from(value: bit_range::ConversionError<I>) -> Self {
        Self::ConversionError {
            cause: value.to_string(),
        }
    }
}

impl<I> From<number::ConversionError<I>> for Error
where
    I: Integral,
{
    fn from(value: number::ConversionError<I>) -> Self {
        Self::ConversionError {
            cause: value.to_string(),
        }
    }
}

impl<U> From<NumberError<U>> for Error
where
    U: Unsigned,
{
    fn from(value: NumberError<U>) -> Self {
        Self::Number {
            source: value.cast(),
        }
    }
}

impl From<TupleError> for Error {
    fn from(value: TupleError) -> Self {
        Self::Tuple { source: value }
    }
}
