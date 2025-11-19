use std::fmt::Debug;

use bytes::{Buf, BytesMut};
use snafu::ResultExt;

use crate::{Reader, io::PartialByte};

use super::ctx;

/// Reference Implementation of the [Reader](crate::Reader)
/// trait.
// #[derive(Debug)]
pub struct ReferenceReader {
    inner: BytesMut,
    partial: PartialByte,
}

impl ReferenceReader {
    /// Constructs a new Reader.
    pub fn new(buf: &[u8]) -> Self {
        Self {
            inner: buf.into(),
            partial: Default::default(),
        }
    }
}

impl Reader for ReferenceReader {
    fn read_bits(&mut self, n: usize) -> Result<bytes::Bytes, super::ReaderError> {
        if n == 0 {
            return Ok(Default::default());
        }

        let bytes = n / 8;
        let bits = n % 8;

        let len = if bits != 0 { bytes + 1 } else { bytes };

        if self.partial.is_on_byte_boundary() {
            let buf = self.read_bytes(len)?;

            if bytes > 0 && bits == 0 {
                // reading only full bytes
                return Ok(buf);
            } else if bytes == 0 && bits > 0 {
                // reading only a partial byte
                // buf is len 1 => read partial byte from it
                let bit = self
                    .partial
                    .set_read(buf[0], bits as u8)
                    .context(ctx::PartialReadSnafu)?;

                return Ok(vec![bit].into());
            }

            // read the last byte partially
            let (full, last) = buf.split_at(buf.len() - 1);
            let partial = self
                .partial
                .set_read(last[0], bits as u8)
                .context(ctx::PartialReadSnafu)?;

            // append the last partial byte to the full bytes
            let res = [full.to_vec(), vec![partial]].concat();

            return Ok(res.into());
        }

        // read with initialized partial

        let rem = self.partial.remaining();
        if n <= rem as usize {
            // only read from partial
            let partial = self.partial.read(n as u8).context(ctx::PartialReadSnafu)?;
            return Ok(vec![partial].into());
        }

        // read the remaining partial bits
        let mut buf = [self.partial.read(rem).context(ctx::PartialReadSnafu)?].to_vec();

        let tail = self.read_bits(n - rem as usize)?;

        buf.extend_from_slice(&tail);

        // shift tail to the left
        for i in 1..buf.len() {
            buf[i - 1] += buf[i] >> rem;
            buf[i] <<= 8 - rem;
        }

        // cut off last byte when shifted empty
        Ok(buf[..len].to_vec().into())
    }

    fn read_bytes(&mut self, n: usize) -> Result<bytes::Bytes, super::ReaderError> {
        snafu::ensure!(
            self.partial.is_on_byte_boundary(),
            ctx::LoosePartialByteSnafu
        );
        snafu::ensure!(
            self.inner.remaining() >= n,
            ctx::MissingBytesSnafu {
                needs: n,
                left: self.inner.remaining()
            }
        );
        Ok(self.inner.copy_to_bytes(n))
    }
}

impl Debug for ReferenceReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader")
            .field("inner", &self.inner.to_vec())
            .field("partial", &self.partial)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use pretty_assertions::assert_eq;

    use crate::io::reader::ReaderError;

    use super::*;

    const BUFFER: &[u8] = &[0b1111_0011, 0b0000_0010, 0b0100_1010];
    // 111 10 01100 000010010 01010

    #[test]
    fn read_bits_test() {
        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bits(0);
        assert_eq!(valid, Ok(Bytes::default()));

        let valid = reader.read_bits(16);
        assert_eq!(valid, Ok(Bytes::from(&BUFFER[..2])));

        let valid = reader.read_bits(4);
        assert_eq!(valid, Ok(Bytes::from([0b0100_0000u8].as_slice())));

        let valid = reader.read_bits(4);
        assert_eq!(valid, Ok(Bytes::from([0b1010_0000].as_slice())));

        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bits(3);
        assert_eq!(valid, Ok(Bytes::from([0b1110_0000].as_slice())));

        let valid = reader.read_bits(2);
        assert_eq!(valid, Ok(Bytes::from([0b1000_0000].as_slice())));

        let valid = reader.read_bits(5);
        assert_eq!(valid, Ok(Bytes::from([0b0110_0000].as_slice())));

        let valid = reader.read_bits(9);
        assert_eq!(valid, Ok(Bytes::from([0b0000_1001, 0].as_slice())));

        let valid = reader.read_bits(5);
        assert_eq!(valid, Ok(Bytes::from([0b0101_0000].as_slice())));

        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bits(13);
        assert_eq!(
            valid,
            Ok(Bytes::from([0b1111_0011, 0b0000_0000].as_slice()))
        );

        let invalid = reader.read_bits(99999);
        assert_eq!(
            invalid,
            Err(ReaderError::MissingBytes {
                needs: 100_000 / 8,
                left: 1
            })
        );
    }

    #[test]
    fn read_bytes_test() {
        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bytes(0);
        assert_eq!(valid, Ok(Bytes::default()));

        let valid = reader.read_bytes(1);
        assert_eq!(valid, Ok(Bytes::from(&BUFFER[..1])));

        let valid = reader.read_bytes(2);
        assert_eq!(valid, Ok(Bytes::from(&BUFFER[1..3])));

        let mut reader = ReferenceReader::new(BUFFER);

        let invalid = reader.read_bytes(9999);
        assert_eq!(
            invalid,
            Err(ReaderError::MissingBytes {
                needs: 9999,
                left: 3
            })
        );

        let _ = reader.read_bits(4);
        let invalid = reader.read_bytes(1);
        assert_eq!(invalid, Err(ReaderError::LoosePartialByte));
    }
}
