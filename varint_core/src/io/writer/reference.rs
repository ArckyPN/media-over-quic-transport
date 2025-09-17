use bytes::{BufMut, BytesMut};

use crate::{Writer, io::PartialByteW};

use super::ctx;

/// Reference Implementation of the [Writer](crate::Writer)
/// trait.
#[derive(Debug, Default, PartialEq)]
pub struct ReferenceWriter {
    inner: BytesMut,
    partial: PartialByteW,
}

impl ReferenceWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: BytesMut::with_capacity(cap),
            partial: PartialByteW::default(),
        }
    }
}

impl Writer for ReferenceWriter {
    fn finish(self) -> Result<bytes::Bytes, super::WriterError> {
        snafu::ensure!(
            self.partial.is_on_byte_boundary(),
            ctx::LoosePartialByteSnafu
        );
        Ok(self.inner.into())
    }

    fn write_bits(&mut self, n: usize, bits: &[u8]) -> &mut Self {
        if n == 0 {
            return self;
        }

        let num_bytes = n / 8;
        let num_bits = n % 8;

        if self.partial.is_on_byte_boundary() {
            let (full, partial) = bits.split_at(num_bytes);

            // write all full bytes
            self.inner.put(full);

            if num_bits > 0 {
                // set partial byte
                self.partial.set(num_bits as u8, partial[0]);
            }
            return self;
        }

        // write all full bytes, apart from the last one, which might be partial
        for byte in bits.iter().take(bits.len() - 1) {
            if let Some(b) = self.partial.set(8, *byte) {
                self.inner.put_u8(b);
            }
        }
        // partially write the final byte
        if let Some(b) = self.partial.set(num_bits as u8, bits[bits.len() - 1]) {
            self.inner.put_u8(b);
        }

        self
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, super::WriterError> {
        snafu::ensure!(
            self.partial.is_on_byte_boundary(),
            ctx::LoosePartialByteSnafu
        );
        self.inner.put(bytes);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER: &[u8] = &[0b1111_0011, 0b0000_0010, 0b0100_1010];

    #[test]
    fn finish_test() {}

    #[test]
    fn write_bits_test() {
        let mut writer = ReferenceWriter::new();

        // writing nothing should have no effect
        writer.write_bits(0, BUFFER);
        assert_eq!(writer, ReferenceWriter::default());

        // write 1111_0011 0000_0010 |
        writer.write_bits(16, BUFFER);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::from(&BUFFER[..2]),
                partial: PartialByteW { byte: 0, index: 0 }
            }
        );

        // write 0100 | 0000
        writer.write_bits(4, &[0b0100_0000]);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::from(&BUFFER[..2]),
                partial: PartialByteW {
                    byte: 0b0100_0000,
                    index: 4
                }
            }
        );

        // write 1010 | 0000
        writer.write_bits(4, &[(BUFFER[2] & 0b0000_1111) << 4]);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::from(BUFFER),
                ..Default::default()
            }
        );

        // start with a fresh writer
        let mut writer = ReferenceWriter::new();

        // write 111 | 0_0000
        writer.write_bits(3, &[0b1110_0000]);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::new(),
                partial: PartialByteW {
                    byte: 0b1110_0000,
                    index: 3
                }
            }
        );

        // previous 111..
        // write 10 | 00_0000
        writer.write_bits(2, &[0b1000_0000]);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::new(),
                partial: PartialByteW {
                    byte: 0b1111_0000,
                    index: 5
                }
            }
        );

        // previous 1111_0..
        // write 0110_0 | 000
        // -> 1111_0011 0
        writer.write_bits(5, &[0b0110_0000]);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::from(&BUFFER[..1]),
                partial: PartialByteW {
                    byte: 0b0000_0000,
                    index: 2
                }
            }
        );

        // previous 0
        // write 0000_1001 0 | 000_0000
        // -> 0000_0100 10
        writer.write_bits(9, &[0b0000_1001, 0]);
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::from(&BUFFER[..2]),
                partial: PartialByteW {
                    byte: 0b0100_0000,
                    index: 3
                }
            }
        );

        // write 0101_0 | 000
        writer.write_bits(5, &[0b0101_0000]);
        // expect 1111_0011 0000_0010 0100_1010
        assert_eq!(
            writer,
            ReferenceWriter {
                inner: BytesMut::from(BUFFER),
                ..Default::default()
            }
        );
    }

    #[test]
    fn write_bytes_test() {
        let mut writer = ReferenceWriter::new();

        writer.write_bytes(&BUFFER[..1]).unwrap();
        assert_eq!(writer.inner, BUFFER[..1]);

        writer.write_bytes(&BUFFER[1..2]).unwrap();
        assert_eq!(writer.inner, BUFFER[..2]);

        writer.write_bytes(&BUFFER[2..]).unwrap();
        assert_eq!(writer.inner, BUFFER);

        // invalid write bytes after partial byte
        let mut writer = ReferenceWriter::new();

        writer.write_bits(4, BUFFER);
        let invalid = writer.write_bytes(&[1, 2, 3]);
        assert!(invalid.is_err());
    }
}
