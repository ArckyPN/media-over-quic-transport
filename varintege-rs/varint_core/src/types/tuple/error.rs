use {
    crate::{BinaryDataError, NumberError, ReaderError, WriterError},
    snafu::Snafu,
};

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[snafu(visibility(pub), module(ctx))]
pub enum TupleError {
    /// Decoding Error
    #[snafu(display("failed to read"))]
    Reader { source: ReaderError },

    /// Encoding Error
    #[snafu(display("failed to write"))]
    Writer { source: WriterError },

    /// Invalid Length Encoding/Decoding
    #[snafu(display("invalid x(i)"))]
    Number { source: NumberError },

    /// Invalid BinaryData Encoding/Decoding
    #[snafu(display("invalid x(b)"))]
    BinaryData { source: BinaryDataError },
}
