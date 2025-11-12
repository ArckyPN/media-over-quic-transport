use {
    super::{BitRange, BitRangeError, ctx},
    crate::{VarInt, VarIntBytes},
    snafu::{OptionExt, ResultExt},
};

impl<const MIN: usize, const MAX: usize> VarInt for BitRange<MIN, MAX> {
    type Error = BitRangeError;

    fn decode<R>(reader: &mut R, length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        let length = length.context(ctx::MissingLengthSnafu)?;

        snafu::ensure!(
            length >= MIN && length <= MAX,
            ctx::InvalidLengthSnafu {
                got: length,
                min: MIN,
                max: MAX
            }
        );

        let buf = reader.read_bits(length).context(ctx::ReaderSnafu)?;

        let mut this = Self::default();
        this.set_bytes(&buf, Some(length))?;

        Ok((this, length))
    }

    fn encode<W>(&self, writer: &mut W, length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let length = length.context(ctx::MissingLengthSnafu)?;
        let bits = self.data.bits();

        writer.write_bits(length, &bits);

        Ok(length)
    }

    fn len_bits(&self) -> Result<usize, Self::Error> {
        Ok(self.data.len())
    }

    fn length_required() -> bool {
        // length is variable and needs to be provided
        true
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{Number, ReferenceReader, ReferenceWriter, VarIntNumber, Writer},
        pretty_assertions::assert_eq,
    };

    const BUF: &[u8] = &[
        // x(i) = 20
        0b0001_0100,
        // 20 bits = 0b0000 0b0000_0000 0b0000_1000 = 8
        0b0000_0000,
        0b0000_0000,
        0b1000_0000, // final 12 bits = 0b0000 0b0001_0000 = 16
        0b0001_0000,
    ];

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUF);
        let mut length = BUF.len() * 8;

        let (len, bits) = Number::decode(&mut reader, None).expect("will fit");
        assert_eq!(bits, 8);
        length -= bits;

        let (valid, bits) =
            BitRange::<8, 32>::decode(&mut reader, Some(len.number())).expect("will fit");
        assert_eq!(bits, len);
        assert_eq!(valid, 8);
        assert_eq!(*valid.bytes(), [0b0000_0000, 0b0000_0000, 0b1000_0000]);
        length -= bits;

        let (valid, bits) = BitRange::<0>::decode(&mut reader, Some(length)).expect("will fit");
        assert_eq!(bits, 12);
        assert_eq!(valid, 16);
        assert_eq!(*valid.bytes(), [0b0000_0001, 0b0000]);
        length -= bits;

        assert!(length == 0);
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();
        let mut length = BUF.len() * 8;

        let len = Number::new(20u8);
        let bits = len.encode(&mut writer, None).expect("will fit");
        assert_eq!(bits, 8);
        length -= bits;

        let num = BitRange::<8, 32>::new_number(8u8, Some(len.number())).expect("will fit");
        let bits = num
            .encode(&mut writer, Some(len.number()))
            .expect("will fit");
        assert_eq!(bits, 20);
        length -= bits;

        let num = BitRange::<0>::new_number(16u8, Some(length)).expect("will fit");
        let bits = num.encode(&mut writer, Some(length)).expect("will fit");
        assert_eq!(bits, 12);
        length -= bits;

        assert!(length == 0);

        assert_eq!(writer.finish().expect("will fit"), BUF);
    }
}
