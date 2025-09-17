mod io;
pub mod types;

pub use io::{
    reader::{Reader, ReferenceReader},
    writer::{ReferenceWriter, Writer},
};
pub use types::{BitNumber, BitRange, Number};

#[cfg(feature = "moq")]
pub use types::{BinaryData, Tuple};

pub trait VarInt {
    /// The data type to decode.
    type Item;

    /// The error type.
    type Error;

    /// Encode the data for transmission.
    fn encode<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: Writer;

    /// Decode received data.
    fn decode<R>(reader: &mut R) -> Result<Self::Item, Self::Error>
    where
        R: Reader;
}
