use std::{fmt::Debug, time::Duration};

use varint::{Error, Reader, VarInt, Writer, x};

use crate::types::parameter::Token;

use super::{decode_bytes, decode_duration, decode_varint};

const DELIVERY_TIMEOUT: u64 = 0x02;
const AUTHORIZATION_TOKEN: u64 = 0x03;
const MAX_CACHE_DURATION: u64 = 0x04;

// TODO there are more params: https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#name-setup-parameters

#[derive(PartialEq, Clone)]
pub enum Parameter {
    /// # Type: even
    ///
    /// Generic parameter Number
    Number(x!(i)),
    /// # Type: odd
    ///
    /// Generic parameter Bytes
    Bytes(x!(b)),
    /// # Type: `0x02`
    ///
    ///
    DeliveryTimeout(Duration),
    /// # Type: `0x03`
    ///
    /// is repeatable
    AuthorizationToken(Token),
    /// # Type: `0x04`
    ///
    ///
    MaxCacheDuration(Duration),
}

impl Parameter {
    pub(crate) fn read<R>(reader: &mut R, ty: u64, bits: &mut usize) -> Result<Self, Error>
    where
        R: Reader,
    {
        Ok(match ty {
            DELIVERY_TIMEOUT => Self::DeliveryTimeout(decode_duration(reader, bits)?),
            AUTHORIZATION_TOKEN => {
                let len = decode_varint(reader, bits)?;
                let (token, len) = Token::decode(reader, Some(len.number::<usize>() * 8))?;
                *bits += len;

                Self::AuthorizationToken(token)
            }
            MAX_CACHE_DURATION => Self::MaxCacheDuration(decode_duration(reader, bits)?),
            x if x.is_multiple_of(2) => Self::Number(decode_varint(reader, bits)?),
            _ => Self::Bytes(decode_bytes(reader, bits)?),
        })
    }

    pub(crate) fn write<W>(&self, writer: &mut W) -> Result<usize, Error>
    where
        W: Writer,
    {
        Ok(match self {
            Self::Number(n) => n.encode(writer, None)?,
            Self::Bytes(b) => b.encode(writer, None)?,
            Self::DeliveryTimeout(d) | Self::MaxCacheDuration(d) => {
                <x!(i)>::try_from(d.as_millis())?.encode(writer, None)?
            }
            Self::AuthorizationToken(t) => {
                let mut bits = 0;
                let len = t.len_bits();
                bits += <x!(i)>::try_from(len / 8)?.encode(writer, None)?;
                bits + t.encode(writer, Some(len))?
            }
        })
    }
}

impl Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => f
                .debug_tuple("Parameter::Number")
                .field(&n.to_string())
                .finish(),
            Self::Bytes(b) => f
                .debug_struct("Parameter::Bytes")
                .field("buffer", b)
                .field("string", &b.to_string())
                .finish(),
            Self::DeliveryTimeout(d) => f
                .debug_tuple("Parameter::DeliveryTimeout")
                .field(d)
                .finish(),
            Self::AuthorizationToken(t) => f.debug_tuple("Parameter::AuthToken").field(t).finish(),
            Self::MaxCacheDuration(d) => f
                .debug_tuple("Parameter::MaxCacheDuration")
                .field(d)
                .finish(),
        }
    }
}
