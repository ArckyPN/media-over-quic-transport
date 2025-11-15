use {
    super::{MAX_U6, MAX_U14, MAX_U30, Number, NumberError, ctx},
    crate::{VarInt, VarIntNumber},
    snafu::ResultExt,
};

impl VarInt for Number {
    type Error = NumberError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        // first byte contains the size and (part
        // of) the number
        let byte = reader.read_bytes(1).context(ctx::ReaderSnafu)?[0];

        // first two bits denote how many bits are
        // part of the number
        let size = (byte & 0b1100_0000) >> 6;

        // start of the number
        let byte = byte & 0b0011_1111;

        let mut vec = vec![byte];
        let bits = match size {
            0b00 => 6,
            0b01 => {
                // one more byte needed
                let ext = reader.read_bytes(1).context(ctx::ReaderSnafu)?;
                vec.extend_from_slice(&ext);
                14
            }
            0b10 => {
                // three more byte needed
                let ext = reader.read_bytes(3).context(ctx::ReaderSnafu)?;
                vec.extend_from_slice(&ext);
                30
            }
            0b11 => {
                // seven more byte needed
                let ext = reader.read_bytes(7).context(ctx::ReaderSnafu)?;
                vec.extend_from_slice(&ext);
                62
            }
            _ => unreachable!("impossible size"),
        };

        // shift left by 2 to account for the 2 size bits
        let len = vec.len();
        for i in 0..len {
            vec[i] <<= 2;
            if i == len - 1 {
                continue;
            }
            vec[i] += vec[i + 1] >> 6;
        }

        // construct the VarInt
        let mut v = Self::default();
        v.data.set_bits(&vec, bits).context(ctx::BitStoreSnafu)?;
        Ok((v, bits + 2))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let value = self.number::<u64>();
        let buf = if value <= MAX_U6 {
            (value as u8).to_be_bytes().to_vec()
        } else if value <= MAX_U14 {
            (0b01 << 14 | (value as u16)).to_be_bytes().to_vec()
        } else if value <= MAX_U30 {
            (0b10 << 30 | (value as u32)).to_be_bytes().to_vec()
        } else {
            (0b11 << 62 | value).to_be_bytes().to_vec()
        };
        writer.write_bytes(&buf).context(ctx::WriterSnafu)?;
        Ok(buf.len() * 8)
    }

    /// Returns the number of bits required to represent
    /// the data as VarInt.
    ///
    /// This function is **Infallible**!
    fn len_bits(&self) -> Result<usize, Self::Error> {
        let value = self.number::<u64>();
        Ok(if value <= MAX_U6 {
            8
        } else if value <= MAX_U14 {
            16
        } else if value <= MAX_U30 {
            32
        } else {
            64
        })
    }

    fn length_required() -> bool {
        // length is provided by the two MSB bits of the first byte
        false
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{ReferenceReader, ReferenceWriter, Writer},
        bytes::Bytes,
    };

    const BUF: &[u8] = &[
        // u6 number = 8
        0b0000_1000,
        // u14 number = 2_048
        0b0100_1000,
        0b0000_0000,
        // u30 number = 524_288
        0b1000_0000,
        0b0000_1000,
        0b0000_0000,
        0b0000_0000,
        // u62 Number = 2_251_799_813_685_248
        0b1100_0000,
        0b0000_1000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ];

    const VALID_NUM_U6: u8 = 8;
    const VALID_NUM_U14: u16 = 2_048;
    const VALID_NUM_U30: u32 = 524_288;
    const VALID_NUM_U62: u64 = 2_251_799_813_685_248;

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUF);

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::from(VALID_NUM_U6), 8)));

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::from(VALID_NUM_U14), 16)));

        let valid = Number::decode(&mut reader, None);
        assert_eq!(valid, Ok((Number::from(VALID_NUM_U30), 32)));

        let valid = Number::decode(&mut reader, None);
        assert_eq!(
            valid,
            Ok((Number::try_from(VALID_NUM_U62).expect("will fit"), 64))
        );
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let valid = Number::from(VALID_NUM_U6).encode(&mut writer, None);
        assert_eq!(valid, Ok(8));

        let valid = Number::from(VALID_NUM_U14).encode(&mut writer, None);
        assert_eq!(valid, Ok(16));

        let valid = Number::from(VALID_NUM_U30).encode(&mut writer, None);
        assert_eq!(valid, Ok(32));

        let valid = Number::try_from(VALID_NUM_U62)
            .expect("will fit")
            .encode(&mut writer, None);
        assert_eq!(valid, Ok(64));

        assert_eq!(writer.finish(), Ok(Bytes::from(BUF.to_vec())))
    }
}
