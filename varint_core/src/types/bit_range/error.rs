use {
    crate::{ReaderError, bitstore},
    funty::Integral,
    snafu::Snafu,
};

#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub), module(ctx))]
pub enum BitRangeError {
    /// Decoding Error
    #[snafu(display("Reader: {source}"))]
    Reader { source: ReaderError },

    /// number needs more bits than available
    #[snafu(display("value {value} needs {needs} bits space, but only {cap} bits can fit"))]
    InvalidCapacity {
        value: u128,
        needs: usize,
        cap: usize,
    },

    /// decoding a [BitRange](crate::BitRange) requires a specified length in bits,
    /// usually provided by a preceding number specifying the
    /// length
    #[snafu(display("decoding requires a length"))]
    MissingLength,

    /// given decode length doesn't fit into the defined BitRange
    #[snafu(display("expected length to be element of [{min}; {max}], but got {got}"))]
    InvalidLength { got: usize, min: usize, max: usize },

    /// Internal Error
    #[snafu(display("BitStore: {source}"))]
    BitStore { source: bitstore::Error },
}

/// Error for TryFrom/Into of [BitRange](crate::BitRange)
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub), module(ctx_conv))]
pub enum BitRangeConversion<I>
where
    I: Integral,
{
    /// Error when trying to create a [BitRange](crate::BitRange) from a negative integer
    #[snafu(display("VarInt Number cannot be negative, trying {value}"))]
    IsNegative { value: I },

    /// Error when [BitRange](crate::BitRange) cannot be created from value
    #[snafu(display("failed to create a BitRange from {value}"))]
    Invalid { value: I, source: BitRangeError },

    /// Error when trying to cast a Number into a too small type
    #[snafu(display("BitRange {value} does not fit into the given type, max value: {max}"))]
    UnFit { value: u128, max: I },
}
