use {
    crate::{BitRangeError, NumberError},
    snafu::Snafu,
};

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[snafu(visibility(pub), module(ctx))]
pub enum IndexMapError {
    Number { source: NumberError },
    KeyValuePair { source: KeyValuePairError },
    Parameter { cause: String },
}

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[snafu(visibility(pub), module(kvp_ctx))]
pub enum KeyValuePairError {
    // VarInt { source: NumberError },
    Number { source: NumberError },
    Bytes { source: BitRangeError },
}
