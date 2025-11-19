use std::time::Duration;

use crate::{NumberError, VarIntNumber};

use crate::{Number, VarInt};

impl VarInt for Duration {
    type Error = NumberError;

    fn decode<R>(reader: &mut R, _length: Option<usize>) -> Result<(Self, usize), Self::Error>
    where
        R: crate::Reader,
    {
        let (num, bits) = Number::decode(reader, None)?;

        Ok((Self::from_millis(num.number()), bits))
    }

    fn encode<W>(&self, writer: &mut W, length: Option<usize>) -> Result<usize, Self::Error>
    where
        W: crate::Writer,
    {
        let mut num = Number::default();
        num.set_number(self.as_millis(), None)?;

        num.encode(writer, length)
    }

    fn len_bits(&self) -> Result<usize, Self::Error> {
        let mut num = Number::default();
        num.set_number(self.as_millis(), None)?;
        num.len_bits()
    }

    fn length_required() -> bool {
        false
    }
}
