use {
    super::{BitNumber, BitNumberError, ctx},
    crate::{VarInt, VarIntNumber},
    snafu::ResultExt,
};

impl<const N: usize, const MIN: u128, const MAX: u128> VarInt for BitNumber<N, MIN, MAX> {
    type Error = BitNumberError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        let buf = reader.read_bits(N).context(ctx::ReaderSnafu)?.to_vec();

        let mut this = Self::default();
        this.data.set_bits(&buf, N).context(ctx::BitStoreSnafu)?;

        let num = this.number::<u128>();
        snafu::ensure!(
            num >= MIN && num <= MAX,
            ctx::OutOfRangeSnafu {
                value: num,
                min: MIN,
                max: MAX
            }
        );

        Ok((this, N))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let buf = self.data.bits();

        writer.write_bits(N, &buf);

        Ok(N)
    }

    /// Returns the number of bits required to represent the data as VarInt.
    ///
    /// This function is **Infallible**!
    fn len_bits(&self) -> Result<usize, Self::Error> {
        Ok(N)
    }

    fn length_required() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{ReferenceReader, ReferenceWriter, Writer, WriterError},
        pretty_assertions::assert_eq,
    };

    const BUF: &[u8] = &[
        0b0000_0000,
        0b0100_0000,
        0b0001_1111,
        0b0011_1000,
        0b0000_0000,
        0b0111_1011,
    ];

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUF);

        let (num, bits) = BitNumber::<20>::decode(&mut reader, None).expect("will fit");
        assert_eq!(bits, 20);
        assert_eq!(num, 1025);

        let (num, bits) = BitNumber::<4>::decode(&mut reader, None).expect("will fit");
        assert_eq!(bits, 4);
        assert_eq!(num, 15);

        let (num, bits) = BitNumber::<5>::decode(&mut reader, None).expect("will fit");
        assert_eq!(bits, 5);
        assert_eq!(num, 7);

        let (num, bits): (BitNumber<19>, _) =
            BitNumber::decode(&mut reader, None).expect("will fit");
        assert_eq!(bits, 19);
        assert_eq!(num, 123);
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let num = BitNumber::<20>::new_number(1025u16, None).expect("will fit");
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(20));

        let num = BitNumber::<4>::new_number(15u8, None).expect("will fit");
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(4));

        let num: BitNumber<5> = BitNumber::new_number(7u32, None).expect("will fit");
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(5));

        let num: BitNumber<19> = BitNumber::new_number(123u64, None).expect("will fit");
        let valid = num.encode(&mut writer, None);
        assert_eq!(valid, Ok(19));

        let valid = writer.finish().expect("will fit");
        assert_eq!(valid, BUF);

        let mut writer = ReferenceWriter::new();
        let valid = BitNumber::<5>::new_number(13u8, None)
            .expect("will fit")
            .encode(&mut writer, None);
        assert_eq!(valid, Ok(5));

        let invalid = writer.finish();
        assert_eq!(invalid, Err(WriterError::LoosePartialByte));
    }
}
