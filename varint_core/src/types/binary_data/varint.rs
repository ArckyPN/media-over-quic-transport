use {
    super::{BinaryData, BinaryDataError, ctx},
    crate::{Number, VarInt, VarIntNumber},
    snafu::ResultExt,
};

impl VarInt for BinaryData {
    type Error = BinaryDataError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        let mut bits = 0;

        // VarInt determines the number of bytes
        let (length, len) = Number::decode(reader, None).context(ctx::NumberSnafu)?;
        bits += len;

        // read the data payload
        let data = reader
            .read_bytes(length.number())
            .context(ctx::ReaderSnafu)?;
        bits += length.number::<usize>() * 8;

        Ok((data.into(), bits))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let len = self.data.len();

        // encode the VarInt length
        let mut bits = Number::new_number(len, None)
            .context(ctx::NumberSnafu)?
            .encode(writer, None)
            .context(ctx::NumberSnafu)?;

        // encode the data payload
        writer.write_bytes(&self.data).context(ctx::WriterSnafu)?;
        bits += len * 8;

        Ok(bits)
    }

    /// Returns the number of bits required to represent the data as VarInt.
    ///
    /// This function is **Infallible**!
    fn len_bits(&self) -> Result<usize, Self::Error> {
        let num_bytes = self.len();
        let len = Number::new_number(num_bytes, None).context(ctx::NumberSnafu)?;

        // data in bits + number of bits required to encode its length
        Ok(num_bytes * 8 + len.len_bits().expect("Number::len_bits is infallible"))
    }

    fn length_required() -> bool {
        // length is provided by a preceding VarInt
        false
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{ReaderError, ReferenceReader, ReferenceWriter, VarIntBytes, Writer},
        bytes::Bytes,
        pretty_assertions::assert_eq,
    };

    const BUF: &[u8] = &[
        // x(i) => 8
        0b0000_1000,
        // 8 bytes data
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    ];

    const INVALID: &[u8] = &[
        // x(i) => 8
        0b0000_1000,
        // only 7 bytes data => too few
        1,
        2,
        3,
        4,
        5,
        6,
        7,
    ];

    #[test]
    fn decode_test() {
        let mut reader = ReferenceReader::new(BUF);

        let valid = BinaryData::decode(&mut reader, None);
        assert_eq!(valid, Ok((BUF[1..].into(), BUF.len() * 8)));

        let mut reader = ReferenceReader::new(INVALID);

        let invalid = BinaryData::decode(&mut reader, None);
        assert_eq!(
            invalid,
            Err(BinaryDataError::Reader {
                source: ReaderError::MissingBytes { needs: 8, left: 7 }
            })
        );
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let data = BinaryData::new_bytes(&BUF[1..], None).expect("infallible");
        let bits = data.encode(&mut writer, None);

        assert_eq!(bits, Ok(BUF.len() * 8));
        assert_eq!(writer.finish(), Ok(Bytes::from(BUF)));
    }
}
