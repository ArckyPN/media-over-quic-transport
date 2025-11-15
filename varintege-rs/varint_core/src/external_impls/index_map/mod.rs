mod error;
mod kvp;

use {
    crate::{Number, Parameter, VarInt, VarIntNumber},
    indexmap::IndexMap,
    snafu::ResultExt,
};

pub use {
    error::{IndexMapError, KeyValuePairError},
    kvp::{KeyValuePair, Value},
};

pub(super) use error::{ctx, kvp_ctx};

impl<V> VarInt for IndexMap<Number, V>
where
    V: Parameter,
{
    type Error = IndexMapError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
        Self: std::marker::Sized,
    {
        let mut bits = 0;
        let mut this = Self::default();

        let (n, len) = Number::decode(reader, None).context(ctx::NumberSnafu)?;
        bits += len;

        for _ in 0..n.number::<usize>() {
            let (typ, len) = KeyValuePair::decode(reader, None).context(ctx::KeyValuePairSnafu)?;
            bits += len;

            this.insert(
                typ.key.clone(),
                V::try_from(typ.clone()).map_err(|_err| IndexMapError::Parameter {
                    cause: format!("failed to created Parameter from {typ:?}"),
                })?,
            );
        }

        Ok((this, bits))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let mut bits = 0;

        let num_params = self.len();
        bits += Number::new_number(num_params, None)
            .context(ctx::NumberSnafu)?
            .encode(writer, None)
            .context(ctx::NumberSnafu)?;

        for (k, v) in self {
            bits += v
                .to_kvp(k.clone())
                .map_err(|err| IndexMapError::Parameter {
                    cause: err.to_string(),
                })?
                .encode(writer, None)
                .context(ctx::KeyValuePairSnafu)?;
        }

        Ok(bits)
    }

    fn len_bits(&self) -> Result<usize, Self::Error> {
        let mut bits = 0;

        let num_params = self.len();
        bits += Number::new_number(num_params, None)
            .context(ctx::NumberSnafu)?
            .len_bits()
            .context(ctx::NumberSnafu)?;

        for (k, v) in self {
            bits += v
                .to_kvp(k.clone())
                .map_err(|err| IndexMapError::Parameter {
                    cause: err.to_string(),
                })?
                .len_bits()
                .context(ctx::KeyValuePairSnafu)?;
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
        crate::{ReferenceReader, ReferenceWriter, VarIntBytes, Writer},
        bytes::Bytes,
        pretty_assertions::assert_eq,
    };

    #[derive(Debug, PartialEq)]
    enum Params {
        Number(Number),
        Bytes(Bytes),
    }

    // should actually be TryFrom but in this case the conversion is infallible
    // so From works
    impl From<KeyValuePair> for Params {
        fn from(value: KeyValuePair) -> Self {
            match value.value {
                Value::Number(value) => Self::Number(value),
                Value::Bytes(value) => Self::Bytes(value.bytes()),
            }
        }
    }

    impl Parameter for Params {
        type PError = String;

        fn to_kvp(&self, key: Number) -> Result<crate::external_impls::KeyValuePair, Self::PError> {
            match (key.number::<u64>(), self) {
                (x, Self::Number(num)) if x.is_multiple_of(2) => Ok(KeyValuePair {
                    key,
                    value: Value::Number(num.clone()),
                }),
                (x, Self::Bytes(buf)) if !x.is_multiple_of(2) => Ok(KeyValuePair {
                    key,
                    value: Value::Bytes(buf.iter().as_slice().into()),
                }),
                _ => Err("invalid encoding".to_owned()),
            }
        }
    }

    const BUF: &[u8] = &[
        7,   // num parameters
        0x0, // number
        5,   //
        0x1, // bytes
        4,   // num bytes
        1, 2, 3, 4,   //
        0x2, // number
        50,  //
        0x3, // bytes
        2,   // num bytes
        0, 6,   //
        0x4, // number
        34,  //
        0x5, // bytes
        4,   // num bytes
        10, 11, 12, 13,  //
        0x6, // number
        0,   //
    ];

    #[test]
    fn varint_test() {
        let mut reader = ReferenceReader::new(BUF);

        let valid = IndexMap::<Number, Params>::decode(&mut reader, None);
        assert_eq!(
            valid,
            Ok((
                IndexMap::from([
                    (0x0u8.into(), Params::Number(5u8.into())),
                    (0x1u8.into(), Params::Bytes([1, 2, 3, 4].to_vec().into())),
                    (0x2u8.into(), Params::Number(50u8.into())),
                    (0x3u8.into(), Params::Bytes([0, 6].to_vec().into())),
                    (0x4u8.into(), Params::Number(34u8.into())),
                    (
                        0x5u8.into(),
                        Params::Bytes([10, 11, 12, 13].to_vec().into())
                    ),
                    (0x6u8.into(), Params::Number(0u8.into())),
                ]),
                BUF.len() * 8
            ))
        );

        let map = valid.expect("is ok").0;
        let mut writer = ReferenceWriter::new();
        let valid = map.encode(&mut writer, None);
        assert_eq!(valid, Ok(BUF.len() * 8));
        assert_eq!(writer.finish(), Ok(BUF.to_vec().into()));

        assert_eq!(map.len_bits(), Ok(BUF.len() * 8));
    }
}
