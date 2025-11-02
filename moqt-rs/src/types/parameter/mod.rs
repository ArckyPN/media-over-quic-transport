use std::{fmt::Debug, time::Duration};

use indexmap::{
    IndexMap,
    map::{IntoIter, Iter, IterMut},
};
use varint::{Error, VarInt, x};

mod param;
mod token;

pub use {param::Parameter, token::Token};

/// ## Parameters Key-Value-Pairs
///
/// Parameters is an abstraction of the
/// Key-Value-Pair Structure from the [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#name-key-value-pair-structure).
#[derive(Clone, Default, PartialEq)]
pub struct Parameters {
    inner: IndexMap<x!(i), Parameter>,
}

impl Parameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Parameter>
    where
        K: Into<x!(i)>,
        V: Into<Parameter>,
    {
        self.inner.insert(key.into(), value.into())
    }

    pub fn get<K>(&self, key: K) -> Option<&Parameter>
    where
        K: Into<x!(i)>,
    {
        self.inner.get(&key.into())
    }

    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut Parameter>
    where
        K: Into<x!(i)>,
    {
        self.inner.get_mut(&key.into())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl IntoIterator for Parameters {
    type Item = (x!(i), Parameter);
    type IntoIter = IntoIter<x!(i), Parameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Parameters {
    type Item = (&'a x!(i), &'a Parameter);
    type IntoIter = Iter<'a, x!(i), Parameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Parameters {
    type Item = (&'a x!(i), &'a mut Parameter);
    type IntoIter = IterMut<'a, x!(i), Parameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl<K, V> From<IndexMap<K, V>> for Parameters
where
    K: Into<x!(i)>,
    V: Into<Parameter>,
{
    fn from(value: IndexMap<K, V>) -> Self {
        Self {
            inner: value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl VarInt for Parameters {
    type Error = Error;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: varint::Reader,
        Self: std::marker::Sized,
    {
        let mut bits = 0;
        let mut this = Self::default();

        let n = decode_varint(reader, &mut bits)?;
        for _ in 0..n.number::<usize>() {
            let ty = decode_varint(reader, &mut bits)?;

            let param = Parameter::read(reader, ty.number(), &mut bits)?;

            // TODO validate if can actually be added
            this.insert(ty, param);
        }

        Ok((this, bits))
    }

    fn encode<W>(&self, writer: &mut W, _length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: varint::Writer,
    {
        let mut bits = 0;

        let num_params = self.len();
        bits += <x!(i)>::try_from(num_params)?.encode(writer, None)?;

        for (k, v) in self {
            bits += k.encode(writer, None)?;
            bits += v.write(writer)?;
        }

        Ok(bits)
    }

    fn len_bits(&self) -> usize {
        let mut bits = 0;

        let num_params = self.len();
        bits += <x!(i)>::try_from(num_params).expect("# TODO ").len_bits();

        for (k, v) in self {
            bits += k.len_bits();
            bits += v.len_bits();
        }

        bits
    }

    fn length_required() -> bool {
        false
    }
}

impl Debug for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.inner.iter().map(|(k, v)| (format!("{k:#X}"), v)))
            .finish()
    }
}

pub(super) fn decode_varint<R>(reader: &mut R, bits: &mut usize) -> Result<x!(i), Error>
where
    R: varint::Reader,
{
    let (num, len) = <x!(i)>::decode(reader, None)?;
    *bits += len;
    Ok(num)
}

pub(super) fn decode_duration<R>(reader: &mut R, bits: &mut usize) -> Result<Duration, Error>
where
    R: varint::Reader,
{
    let num = decode_varint(reader, bits)?;
    Ok(Duration::from_millis(num.number()))
}

pub(super) fn decode_bytes<R>(reader: &mut R, bits: &mut usize) -> Result<x!(b), Error>
where
    R: varint::Reader,
{
    let (buf, len) = <x!(b)>::decode(reader, None)?;
    *bits += len;

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use varint::core::ReferenceReader;

    use super::*;

    // TODO use TestData impl

    const BUF: &[u8] = &[
        7,   // num parameters
        0x0, // generic number
        5,   //
        0x1, // generic bytes
        4,   // num bytes
        1, 2, 3, 4,   //
        0x2, // delivery timeout
        50,  //
        0x3, // auth token
        2,   // num bytes
        0,   // delete type
        6,   // alias
        0x4, // max cache
        34,  //
        0x5, // generic bytes
        4,   // num bytes
        10, 11, 12, 13,  //
        0x6, // generic number
        0,   //
    ];

    #[test]
    fn varint_test() {
        let mut reader = ReferenceReader::new(BUF);

        let valid = Parameters::decode(&mut reader, Some(BUF.len() * 8));
        assert_eq!(
            valid,
            Ok((
                Parameters {
                    inner: IndexMap::from([
                        (0u8.into(), Parameter::Number(5u8.into())),
                        (1u8.into(), Parameter::Bytes(vec![1, 2, 3, 4].into())),
                        (
                            2u8.into(),
                            Parameter::DeliveryTimeout(Duration::from_millis(50))
                        ),
                        (
                            3u8.into(),
                            Parameter::AuthorizationToken(Token::new_delete(6u8))
                        ),
                        (
                            4u8.into(),
                            Parameter::MaxCacheDuration(Duration::from_millis(34))
                        ),
                        (5u8.into(), Parameter::Bytes(vec![10, 11, 12, 13].into())),
                        (6u8.into(), Parameter::Number(0u8.into()))
                    ])
                },
                BUF.len() * 8
            ))
        );
    }
}
