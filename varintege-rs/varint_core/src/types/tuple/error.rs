use {
    crate::{BinaryDataError, NumberError, ReaderError, WriterError},
    snafu::Snafu,
};

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[snafu(visibility(pub), module(ctx))]
pub enum TupleError {
    /// Decoding Error
    #[snafu(display("Reader: {source}"))]
    Reader { source: ReaderError },

    /// Encoding Error
    #[snafu(display("Writer: {source}"))]
    Writer { source: WriterError },

    /// Invalid Length Encoding/Decoding
    #[snafu(display("VarInt: {source}"))]
    Number { source: NumberError },

    /// Invalid BinaryData Encoding/Decoding
    #[snafu(display("BinaryData: {source}"))]
    BinaryData { source: BinaryDataError },
}
