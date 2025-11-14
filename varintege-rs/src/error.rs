use funty::Integral;
use snafu::Snafu;
use varint_core::{
    BinaryDataError, BitNumberError, BitRangeError, NumberError, TupleError,
    external_impls::IndexMapError,
    types::{BitNumberConversion, BitRangeConversion, NumberConversion},
};

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[snafu(visibility(pub), module(ctx))]
pub enum Error {
    // TODO moq feature
    #[snafu(display("BinaryData: {source}"))]
    BinaryData { source: BinaryDataError },
    #[snafu(display("BitNumber: {source}"))]
    BitNumber { source: BitNumberError },
    #[snafu(display("BitRange: {source}"))]
    BitRange { source: BitRangeError },
    #[snafu(display("Conversion: {cause}"))]
    ConversionError { cause: String },
    #[snafu(display("expected to decode {expected} bits, but read {got}"))]
    LengthMismatch { expected: usize, got: usize },
    #[snafu(display("a length was expected by the type"))]
    MissingLength,
    #[snafu(display("Number: {source}"))]
    Number { source: NumberError },
    // TODO moq feature
    #[snafu(display("Tuple: {source}"))]
    Tuple { source: TupleError },
    // TODO moq feature
    #[snafu(display("IndexMap: {source}"))]
    IndexMap { source: IndexMapError },
    #[snafu(display("Unknown Value: got {value}"))]
    UnknownValue { value: u128 },
}

impl From<BinaryDataError> for Error {
    fn from(value: BinaryDataError) -> Self {
        Self::BinaryData { source: value }
    }
}

impl From<BitNumberError> for Error {
    fn from(value: BitNumberError) -> Self {
        Self::BitNumber { source: value }
    }
}

impl From<BitRangeError> for Error {
    fn from(value: BitRangeError) -> Self {
        Self::BitRange { source: value }
    }
}

impl<I> From<BitNumberConversion<I>> for Error
where
    I: Integral,
{
    fn from(value: BitNumberConversion<I>) -> Self {
        Self::ConversionError {
            cause: value.to_string(),
        }
    }
}

impl<I> From<BitRangeConversion<I>> for Error
where
    I: Integral,
{
    fn from(value: BitRangeConversion<I>) -> Self {
        Self::ConversionError {
            cause: value.to_string(),
        }
    }
}

impl<I> From<NumberConversion<I>> for Error
where
    I: Integral,
{
    fn from(value: NumberConversion<I>) -> Self {
        Self::ConversionError {
            cause: value.to_string(),
        }
    }
}

impl From<NumberError> for Error {
    fn from(value: NumberError) -> Self {
        Self::Number { source: value }
    }
}

impl From<TupleError> for Error {
    fn from(value: TupleError) -> Self {
        Self::Tuple { source: value }
    }
}

impl From<IndexMapError> for Error {
    fn from(value: IndexMapError) -> Self {
        Self::IndexMap { source: value }
    }
}
