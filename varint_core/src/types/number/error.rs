use {
    super::{MAX_U62, Number},
    crate::{ReaderError, StringError, WriterError, bitstore},
    funty::Integral,
    snafu::Snafu,
};

#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub), module(ctx))]
pub enum NumberError {
    /// Decoding Error
    #[snafu(display("Reader: {source}"))]
    Reader { source: ReaderError },

    /// Encoding Error
    #[snafu(display("Writer: {source}"))]
    Writer { source: WriterError },

    /// Internal Error
    #[snafu(display("BitStore: {source}"))]
    BitStore { source: bitstore::Error },

    /// Trying to create a VarInt with a too large value
    #[snafu(display("number {num} is too large, max={MAX_U62}"))]
    TooLarge { num: u128 },

    /// Failed FromStr
    #[snafu(display("FromString: {source}"))]
    String { source: StringError },
}

/// Error for TryFrom/Into of [VarInt](crate::Number)
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub), module(ctx_conv))]
pub enum NumberConversion<I>
where
    I: Integral,
{
    /// Error when trying to create a [Number] from a negative integer
    #[snafu(display("VarInt Number cannot be negative, trying {value}"))]
    IsNegative { value: I },

    /// Error when [VarInt](crate::Number) cannot be created from value
    #[snafu(display("failed to create a VarInt Number from {value}"))]
    Invalid { value: I, source: NumberError },

    /// Error when trying to cast a Number into a too small type
    #[snafu(display("Number {value} does not fit into the given type, max value: {max}"))]
    UnFit { value: Number, max: I },
}
