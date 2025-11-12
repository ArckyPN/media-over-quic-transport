use {
    crate::{NumberError, ReaderError, WriterError},
    snafu::Snafu,
};

#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub), module(ctx))]
pub enum BinaryDataError {
    /// Decoding Error
    #[snafu(display("Reader: {source}"))]
    Reader { source: ReaderError },

    /// Encoding Error
    #[snafu(display("Writer: {source}"))]
    Writer { source: WriterError },

    /// Invalid Length Encoding/Decoding
    #[snafu(display("Writer: {source}"))]
    Number { source: NumberError },
}
