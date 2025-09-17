use bytes::{Buf, BytesMut};
use snafu::ResultExt;

use crate::{
    Reader,
    io::{PartialByteR, partial},
};

use super::{ReaderError, ctx};

/// Reference Implementation of the [Reader](crate::Reader)
/// trait.
#[derive(Debug)]
pub struct ReferenceReader {
    inner: BytesMut,
    partial: PartialByteR,
}

impl ReferenceReader {
    /// Constructs a new Reader.
    pub fn new(buf: &[u8]) -> Self {
        Self {
            inner: buf.into(),
            partial: PartialByteR::default(),
        }
    }

    /// Shifts all bits to the right.
    fn shift_partial_read(buf: &mut [u8], shift: u8) -> Result<(), ReaderError> {
        snafu::ensure!(buf.len() > 1, ctx::InvalidShiftSnafu);
        partial::valid_n(shift).context(ctx::PartialReadSnafu)?;

        // loop over buf in reverse until the second element
        for i in (1..buf.len()).rev() {
            // add the shifted previous byte
            buf[i] += PartialByteR::remove_bits(buf[i - 1], 8 - shift)
                .context(ctx::PartialReadSnafu)?
                << (8 - shift);
            // shift the previous byte
            buf[i - 1] >>= shift;
        }
        Ok(())
    }
}

impl Reader for ReferenceReader {
    fn read_bits(&mut self, n: usize) -> Result<bytes::Bytes, super::ReaderError> {
        if n == 0 {
            return Ok(Default::default());
        }

        let bytes = n / 8;
        let bits = n % 8;

        if self.partial.is_on_byte_boundary() {
            let len = if bits != 0 { bytes + 1 } else { bytes };

            let buf = self.read_bytes(len)?;

            if bytes > 0 && bits == 0 {
                // reading only full bytes
                return Ok(buf);
            } else if bytes == 0 && bits > 0 {
                // reading only a partial byte
                // buf is len 1 => read partial byte from it
                let bit = self
                    .partial
                    .set(bits as u8, buf[0])
                    .context(ctx::PartialReadSnafu)?;
                return Ok(vec![bit].into());
            }

            // separate the partial byte
            let (full, partial) = buf.split_at(buf.len() - 1);
            let partial = self
                .partial
                .set(bits as u8, partial[0])
                .context(ctx::PartialReadSnafu)?;

            let mut res = [full.to_vec(), vec![partial]].concat();
            Self::shift_partial_read(&mut res, 8 - bits as u8)?;

            return Ok(res.into());
        }

        // read with initialized partial

        // read just from partial
        if bytes == 0 && bits as u8 <= self.partial.remaining() {
            let partial = self
                .partial
                .get(bits as u8)
                .context(ctx::PartialReadSnafu)?;
            return Ok(vec![partial].into());
        }

        // read past stored partial

        // read remaining partial
        let rem = self.partial.remaining();
        // TODO this looks weird
        let buf = if n > rem as usize {
            vec![self.partial.get(rem).context(ctx::PartialReadSnafu)?]
        } else {
            // seems to never reach this case
            Default::default()
        };

        // back at a byte boundary
        let rem = n - rem as usize;
        let bits = rem % 8;

        let res = self.read_bits(rem)?;
        let mut res = [buf, res.to_vec()].concat();
        Self::shift_partial_read(&mut res, 8 - bits as u8)?;

        // first byte will be always 0, remove it
        Ok(res[1..].to_vec().into())
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

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER: &[u8] = &[0b1111_0011, 0b0000_0010, 0b0100_1010];
    // 111 10 01100 000010010 01010

    #[test]
    fn read_bits_test() {
        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bits(0).unwrap();
        assert!(valid.is_empty());

        let valid = reader.read_bits(16).unwrap();
        assert_eq!(valid, BUFFER[..2]);

        let valid = reader.read_bits(4).unwrap();
        assert_eq!(valid[0], BUFFER[2] >> 4);

        let valid = reader.read_bits(4).unwrap();
        assert_eq!(valid[0], BUFFER[2] & 0b1111);

        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bits(3).unwrap();
        assert_eq!(valid[0], BUFFER[0] >> 5);

        let valid = reader.read_bits(2).unwrap();
        assert_eq!(valid[0], (BUFFER[0] & 0b1_1111) >> 3);

        let valid = reader.read_bits(5).unwrap();
        assert_eq!(valid[0], 0b1100);

        let valid = reader.read_bits(9).unwrap();
        assert_eq!(valid[0], 0b10010);

        let valid = reader.read_bits(5).unwrap();
        assert_eq!(valid[0], BUFFER[2] & 0b1_1111);

        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bits(13).unwrap();
        assert_eq!(valid, vec![0b1_1110, 0b0110_0000]);

        let invalid = reader.read_bits(99999);
        assert!(invalid.is_err());
    }

    #[test]
    fn read_bytes_test() {
        let mut reader = ReferenceReader::new(BUFFER);

        let valid = reader.read_bytes(0).unwrap();
        assert!(valid.is_empty());

        let valid = reader.read_bytes(1).unwrap();
        assert_eq!(valid, BUFFER[..1]);

        let valid = reader.read_bytes(2).unwrap();
        assert_eq!(valid, BUFFER[1..3]);

        let mut reader = ReferenceReader::new(BUFFER);

        let invalid = reader.read_bytes(9999);
        assert!(invalid.is_err());

        reader.read_bits(4).unwrap();
        let invalid = reader.read_bytes(1);
        assert!(invalid.is_err());
    }
}
