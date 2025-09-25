pub(crate) mod bitstore;
mod io;
pub mod types;

pub use io::{
    reader::{Reader, ReferenceReader},
    writer::{ReferenceWriter, Writer},
};
pub use types::{BitNumber, BitNumberError, BitRange, BitRangeNumberError, Number, NumberError};

#[cfg(feature = "moq")]
pub use types::{BinaryData, Tuple};

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
}

// TODO get rid of all unwraps
// TODO impl VarInt for primitive types, which make sense
// TODO rework all Error types, every fn should have its own Error type, errors can be reused if fns share the same
