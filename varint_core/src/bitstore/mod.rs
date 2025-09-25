mod utils;

use std::fmt::Debug;

use funty::Unsigned;

use bytes::Bytes;
use snafu::{ResultExt, Snafu};

#[derive(Clone, PartialEq, PartialOrd)]
pub struct BitStore<const MIN: usize = 0, const MAX: usize = { usize::MAX }> {
    data: Bytes,
    len: usize,
}

impl<const MIN: usize, const MAX: usize> BitStore<MIN, MAX> {
    pub fn set_bits(&mut self, buf: &[u8], bits: usize) -> Result<&mut Self, Error> {
        Self::ensure_fit(bits).context(InvalidLengthSnafu)?;

        self.data = Bytes::copy_from_slice(buf);
        self.len = bits;

        Ok(self)
    }

    pub fn set_number<U>(&mut self, num: U, bits: Option<usize>) -> Result<&mut Self, Error>
    where
        U: Unsigned,
    {
        let num_bits = utils::num_bits(num);
        let bits = match bits {
            Some(b) => {
                // check if number can fit into the expected space
                if b < num_bits {
                    return Err(Error::InvalidFit {
                        num: num.as_u128(),
                        needs: num_bits,
                        expected: b,
                    });
                }
                b
            }
            None => num_bits,
        };
        Self::ensure_fit(bits).context(InvalidLengthSnafu)?;

        self.len = bits;

        // remove superfluous preceding bytes
        let bytes = if self.len % 8 != 0 {
            self.len / 8 + 1
        } else {
            self.len / 8
        };

        let mut bytes = match self.len {
            0 => todo!("Error or what to do in this case?"),
            1..9 => num.as_u8().to_be_bytes().to_vec(),
            9..17 => num.as_u16().to_be_bytes()[2 - bytes..].to_vec(),
            17..33 => num.as_u32().to_be_bytes()[4 - bytes..].to_vec(),
            33..65 => num.as_u64().to_be_bytes()[8 - bytes..].to_vec(),
            65..129 => num.as_u128().to_be_bytes()[16 - bytes..].to_vec(),
            _ => return Err(Error::TooLarge),
        };

        if self.len % 8 != 0 {
            utils::shift_bits(&mut bytes, self.len);
        }

        self.data = Bytes::copy_from_slice(&bytes);

        Ok(self)
    }

    pub fn bits(&self) -> Bytes {
        self.data.clone()
    }

    pub fn number<U>(&self) -> U
    where
        U: Unsigned,
    {
        let mut buf = self.data.to_vec();
        utils::unshift_bits(&mut buf, self.len);

        let len = buf.len();
        match len {
            0 => U::try_from(0u8).unwrap_or_default(),
            1 => U::try_from(u8::from_be_bytes(buf.try_into().expect("len 1 verified")))
                .unwrap_or_default(),
            2 => U::try_from(u16::from_be_bytes(buf.try_into().expect("len 2 verified")))
                .unwrap_or_default(),
            3..5 => {
                let mut vec = vec![0; 4 - len];
                vec.extend_from_slice(&buf);
                U::try_from(u32::from_be_bytes(vec.try_into().expect("len 4 verified")))
                    .unwrap_or_default()
            }
            5..9 => {
                let mut vec = vec![0; 8 - len];
                vec.extend_from_slice(&buf);
                U::try_from(u64::from_be_bytes(vec.try_into().expect("len 8 verified")))
                    .unwrap_or_default()
            }
            9..17 => {
                let mut vec = vec![0; 16 - len];
                vec.extend_from_slice(&buf);
                U::try_from(u128::from_be_bytes(
                    vec.try_into().expect("len 16 verified"),
                ))
                .unwrap_or_default()
            }
            _ => unreachable!("number above u128 are not supported"),
        }
    }

    fn ensure_fit(bits: usize) -> Result<(), InvalidBitLength> {
        if bits < MIN || bits > MAX {
            Err(InvalidBitLength {
                got: bits,
                min: MIN,
                max: MAX,
            })
        } else {
            Ok(())
        }
    }
}

impl<const MIN: usize, const MAX: usize> Default for BitStore<MIN, MAX> {
    fn default() -> Self {
        let bytes = MIN / 8;
        let bits = MIN % 8;

        let len = if bits != 0 { bytes + 1 } else { bytes };

        // init with minimum number of bytes allocated
        Self {
            data: Bytes::copy_from_slice(&vec![0; len]),
            len: 0,
        }
    }
}

impl<const MIN: usize, const MAX: usize> Debug for BitStore<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitStore")
            .field("len", &self.len)
            .field(
                "binary",
                &self
                    .data
                    .iter()
                    .map(|byte| format!("{byte:08b}"))
                    .collect::<Vec<String>>()
                    .join(" "),
            )
            .finish()
    }
}

#[derive(Debug, Snafu, Clone, PartialEq)]
pub enum Error {
    /// length is not between the required bounds
    #[snafu(display("invalid length"))]
    InvalidLength { source: InvalidBitLength },
    /// trying to fit a number into too small a space
    #[snafu(display(
        "number >{num}< needs >{needs}< bits, but tried to fit into >{expected}< bits"
    ))]
    InvalidFit {
        num: u128,
        needs: usize,
        expected: usize,
    },
    /// trying to set a number beyond 128 bits
    #[snafu(display("numbers larger than u128 are not supported"))]
    TooLarge,
}

/// length is not between the required bounds
#[derive(Debug, Snafu, Clone, Copy, PartialEq)]
#[snafu(display("invalid length, got >{got}<, but need between >{min}< and >{max}<"))]
pub struct InvalidBitLength {
    got: usize,
    min: usize,
    max: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_test() {
        let store = BitStore::<8, 16>::default();
        assert_eq!(store.data.len(), 1);
        assert_eq!(*store.data, [0]);

        let store = BitStore::<12, 64>::default();
        assert_eq!(store.data.len(), 2);
        assert_eq!(*store.data, [0, 0]);
    }

    #[test]
    fn set_bits_test() {
        let mut store = BitStore::<0, 16>::default();

        store.set_bits(&[0b1100_0000], 5).unwrap();
        assert_eq!(store.number::<u8>(), 0b0001_1000);
        assert_eq!(*store.bits(), [0b1100_0000]);
    }

    #[test]
    fn set_number_test() {
        let mut store = BitStore::<0, 160>::default();

        store.set_number(4u8, Some(5)).unwrap();
        assert_eq!(store.number::<u8>(), 4);
        assert_eq!(*store.bits(), [0b0010_0000]);

        store.set_number(4u8, Some(8)).unwrap();
        assert_eq!(store.number::<u8>(), 4);
        assert_eq!(*store.bits(), [4]);

        store.set_number(42u16, Some(11)).unwrap();
        assert_eq!(store.number::<u32>(), 42);
        assert_eq!(*store.bits(), [0b0000_0101, 0b0100_0000]);

        store.set_number(512u16, Some(15)).unwrap();
        assert_eq!(store.number::<u32>(), 512);
        assert_eq!(*store.bits(), [0b0000_0100, 0b0000_0000]);
    }
}
