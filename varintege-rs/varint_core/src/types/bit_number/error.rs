use {
    crate::{ReaderError, bitstore},
    funty::Integral,
    snafu::Snafu,
};

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[snafu(visibility(pub), module(ctx))]
pub enum BitNumberError {
    /// Decoding Error
    #[snafu(display("failed to read"))]
    Reader { source: ReaderError },

    /// Internal Error
    #[snafu(display("invalid BitStore"))]
    BitStore { source: bitstore::Error },

    /// number is outside of the specified range
    #[snafu(display("value {value} doesn't fit into specified range [{min}; {max}]"))]
    OutOfRange { value: u128, min: u128, max: u128 },

    /// number needs more bits than available
    #[snafu(display("value {value} needs {needs} bits space, but only {cap} bits can fit"))]
    InvalidCapacity {
        value: u128,
        needs: usize,
        cap: usize,
    },
}

/// Error for TryFrom/Into of [BitNumber](crate::BitNumber)
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub), module(ctx_conv))]
pub enum BitNumberConversion<I>
where
    I: Integral,
{
    /// Error when trying to create a [BitNumber](crate::BitNumber) from a negative integer
    #[snafu(display("VarInt Number cannot be negative, trying {value}"))]
    IsNegative { value: I },

    /// Error when [BitNumber](crate::BitNumber) cannot be created from value
    #[snafu(display("failed to create a BitNumber from {value}"))]
    Invalid { value: I, source: BitNumberError },

    /// Error when trying to cast a Number into a too small type
    #[snafu(display("BitNumber {value} does not fit into the given type, max value: {max}"))]
    UnFit { value: u128, max: I },
}
