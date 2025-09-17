mod reference;

pub use reference::ReferenceWriter;

use bytes::Bytes;
use snafu::Snafu;

use crate::io::PartialByteError;

/// The Writer Trait need for the [VarInt](crate::VarInt::encode)
/// encoding function.
///
/// [Reference](crate::ReferenceWriter) implementation
pub trait Writer {
    /// Finalize the writing process, returning the written buffer.
    ///
    /// Guaranteed to end on a Byte boundary.
    fn finish(self) -> Result<Bytes, WriterError>;

    /// This should guarantee to always write `n` Bits
    /// from `bits[..n]`.
    ///
    /// When writing a partial Byte, the next write operation must
    /// continue where the previous write ended.
    ///
    /// Before using next `write_bytes` or `finish` this must have ended on
    /// a Byte boundary.
    ///
    /// Partial bytes should remain in an internal buffer until a
    /// byte boundary has been reached.
    ///
    /// # Example
    ///
    /// To write a partial byte with just bits, the partial byte must
    /// be in Big Endian.
    ///
    /// To write the decimal 29 (0b0001_1101) the byte must be shifted
    /// to the left by 3 => 0b1110_1000 and then only 5 bits are to
    /// be written:
    ///
    /// ```ignore
    /// let decimal = 0b0001_1101 << 3; // 0b0001_1101 << 3 = 0b1110_1000
    /// writer.write_bits(5, &[decimal]);
    /// ```
    fn write_bits(&mut self, n: usize, bits: &[u8]) -> &mut Self;

    /// On success this should guarantee to always write `n` Bytes.
    ///
    /// This should always start on a Byte boundary. An Error
    /// must be returned when the previous write didn't end on
    /// on a byte boundary.
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, WriterError>;
}

/// Error returned by [Writer](crate::Writer).
#[derive(Debug, Snafu, PartialEq, Clone)]
#[snafu(visibility(pub(super)), module(ctx))]
pub enum WriterError {
    /// previous [write_bits](crate::Writer::write_bits) didn't end
    /// on a byte boundary
    #[snafu(display("the previous write left a loose partial byte"))]
    LoosePartialByte,
    /// error related to a partial byte read
    #[snafu(display("invalid partial byte read"))]
    PartialRead { source: PartialByteError },
}
