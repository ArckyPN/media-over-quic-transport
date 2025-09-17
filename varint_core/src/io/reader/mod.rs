mod reference;

pub use reference::ReferenceReader;

use bytes::Bytes;
use snafu::Snafu;

use super::partial::PartialByteError;

/// The Reader Trait need for the [VarInt](crate::VarInt::decode)
/// decoding method.
///
/// [Reference](crate::ReferenceReader) implementation
pub trait Reader {
    /// On success this should guarantee to always read `n` Bits.
    ///
    /// When reading a partial Byte, the next read operation must
    /// continue where the previous read ended.
    ///
    /// Before using next `read_bytes` this must have ended on
    /// a Byte boundary.
    fn read_bits(&mut self, n: usize) -> Result<Bytes, ReaderError>;

    /// On success this should guarantee to always read `n` Bytes.
    ///
    /// This should always start on a Byte boundary. An Error
    /// must be returned when the internal partial bit index
    /// signifies to continue at a partial Byte.
    fn read_bytes(&mut self, n: usize) -> Result<Bytes, ReaderError>;
}

/// Error returned by [Reader](crate::Reader).
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub(super)), module(ctx))]
pub enum ReaderError {
    /// trying to read more bytes than available
    #[snafu(display("tried to read >{needs}< bytes from >{left}< bytes buffer"))]
    MissingBytes { needs: usize, left: usize },
    /// previous [read_bits](crate::Reader::read_bits) didn't end
    /// on a byte boundary
    #[snafu(display("the previous read left a loose partial byte"))]
    LoosePartialByte, // TODO integrate this into PartialRead
    /// error related to a partial byte read
    #[snafu(display("invalid partial byte read"))]
    PartialRead { source: PartialByteError },
    #[snafu(display("can only shift vectors with at least 2 elements"))]
    InvalidShift,
}
