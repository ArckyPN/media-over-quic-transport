use {
    super::{KeyValuePairError, kvp_ctx as ctx},
    crate::{BitRange, Number, VarInt, VarIntNumber},
    snafu::ResultExt,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Value {
    Number(Number),
    Bytes(BitRange),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct KeyValuePair {
    pub key: Number,
    pub value: Value,
}

impl VarInt for KeyValuePair {
    type Error = KeyValuePairError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        let mut bits = 0;

        let (key, len) = Number::decode(reader, None).context(ctx::NumberSnafu)?;
        bits += len;

        match key.number::<u128>() {
            x if x.is_multiple_of(2) => {
                let (num, len) = Number::decode(reader, None).context(ctx::NumberSnafu)?;
                bits += len;

                Ok((
                    Self {
                        key,
                        value: Value::Number(num),
                    },
                    bits,
                ))
            }
            _ => {
                let (num, len) = Number::decode(reader, None).context(ctx::NumberSnafu)?;
                bits += len;

                let (bytes, len) = BitRange::decode(reader, Some(num.number::<usize>() * 8))
                    .context(ctx::BytesSnafu)?;
                bits += len;

                Ok((
                    Self {
                        key,
                        value: Value::Bytes(bytes),
                    },
                    bits,
                ))
            }
        }
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let mut bits = 0;

        bits += self.key.encode(writer, None).context(ctx::NumberSnafu)?;

        match self {
            Self {
                value: Value::Number(num),
                ..
            } => {
                bits += num.encode(writer, None).context(ctx::NumberSnafu)?;
            }
            Self {
                value: Value::Bytes(buf),
                ..
            } => {
                let length = buf.len_bits().context(ctx::BytesSnafu)?;
                bits += Number::new_number(length / 8, None)
                    .context(ctx::NumberSnafu)?
                    .encode(writer, None)
                    .context(ctx::NumberSnafu)?;
                bits += buf.encode(writer, Some(length)).context(ctx::BytesSnafu)?;
            }
        }

        Ok(bits)
    }

    fn len_bits(&self) -> Result<usize, Self::Error> {
        let mut bits = 0;

        bits += self.key.len_bits().context(ctx::NumberSnafu)?;

        match self {
            Self {
                value: Value::Number(num),
                ..
            } => {
                bits += num.len_bits().context(ctx::NumberSnafu)?;
            }
            Self {
                value: Value::Bytes(buf),
                ..
            } => {
                let length = buf.len_bits().context(ctx::BytesSnafu)?;
                bits += Number::new_number(length / 8, None)
                    .context(ctx::NumberSnafu)?
                    .len_bits()
                    .context(ctx::NumberSnafu)?;
                bits += buf.len_bits().context(ctx::BytesSnafu)?;
            }
        }

        Ok(bits)
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
        pretty_assertions::assert_eq,
    };

    const BUF: &[u8] = &[
        0x0, // number
        5,   //
        0x1, // bytes
        4,   // num bytes
        1, 2, 3, 4, //
    ];

    #[test]
    fn varint_test() {
        let mut reader = ReferenceReader::new(BUF);

        let valid = KeyValuePair::decode(&mut reader, None);
        assert_eq!(
            valid,
            Ok((
                KeyValuePair {
                    key: 0u8.into(),
                    value: Value::Number(5u8.into())
                },
                16
            ))
        );
        let kvp1 = valid.expect("is ok").0;

        let valid = KeyValuePair::decode(&mut reader, None);
        assert_eq!(
            valid,
            Ok((
                KeyValuePair {
                    key: 1u8.into(),
                    value: Value::Bytes([1, 2, 3, 4].into())
                },
                48
            ))
        );
        let kvp2 = valid.expect("is ok").0;

        let mut writer = ReferenceWriter::new();

        let valid = kvp1.encode(&mut writer, None);
        assert_eq!(valid, Ok(16));

        let valid = kvp2.encode(&mut writer, None);
        assert_eq!(valid, Ok(48));

        assert_eq!(writer.finish(), Ok(BUF.to_vec().into()));
    }
}
