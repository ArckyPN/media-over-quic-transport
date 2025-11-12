use {
    super::{Tuple, TupleError, ctx},
    crate::{BinaryData, Number, VarInt, VarIntNumber},
    snafu::ResultExt,
};

impl VarInt for Tuple {
    type Error = TupleError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        let mut bits = 0;
        let (length, len) = Number::decode(reader, None).context(ctx::NumberSnafu)?;
        bits += len;

        let mut tuples = Vec::new();

        for _ in 0..length.number::<usize>() {
            let (data, b) = BinaryData::decode(reader, None).context(ctx::BinaryDataSnafu)?;
            bits += b;

            tuples.push(data);
        }

        Ok((tuples.into(), bits))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let len = self.data.len();
        let length = Number::new_number(len as u64, None).context(ctx::NumberSnafu)?;

        let mut bits = length.encode(writer, None).context(ctx::NumberSnafu)?;

        for tuple in &self.data {
            let b = tuple.encode(writer, None).context(ctx::BinaryDataSnafu)?;
            bits += b;
        }

        Ok(bits)
    }

    fn len_bits(&self) -> Result<usize, Self::Error> {
        let num_tuples = self.len();

        let len = Number::new_number(num_tuples, None).context(ctx::NumberSnafu)?;

        // each Binary data bits + number of bits required to encode the number of BinaryData
        Ok(self.data.iter().fold(0, |acc, bd| {
            acc + bd.len_bits().expect("BinaryData::len_bits is infallible")
        }) + len.len_bits().expect("Number::len_bits is infallible"))
    }

    fn length_required() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{ReferenceReader, ReferenceWriter, Writer},
        bytes::Bytes,
        pretty_assertions::assert_eq,
    };

    const BUF: &[u8] = &[
        // x(i) = 2 tuples
        0b0000_0010,
        // x(i) = 3
        0b0000_0011,
        // 3 bytes, first BinaryData
        1,
        2,
        3,
        // x(i) = 5
        0b0000_0101,
        // 5 bytes, second BinaryData
        1,
        2,
        3,
        4,
        5,
    ];

    #[test]
    fn decode_test() {
        use pretty_assertions::assert_eq;
        let mut reader = ReferenceReader::new(BUF);

        let valid = Tuple::decode(&mut reader, None);
        assert_eq!(
            valid,
            Ok((Tuple::from([&BUF[2..5], &BUF[6..]]), BUF.len() * 8))
        );
    }

    #[test]
    fn encode_test() {
        let mut writer = ReferenceWriter::new();

        let tuple = Tuple::from([&BUF[2..5], &BUF[6..]]);

        let bits: Result<usize, TupleError> = tuple.encode(&mut writer, None);
        assert_eq!(bits, Ok(BUF.len() * 8));
        assert_eq!(writer.finish(), Ok(Bytes::from(BUF)));
    }
}
