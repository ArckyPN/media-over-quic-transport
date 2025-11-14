use std::fmt::Debug;

use snafu::Snafu;

/// Partial Reader and Writer
///
/// Used by the reference [Reader](crate::ReferenceReader)
/// and [Writer](crate::ReferenceWriter).
#[derive(Default, PartialEq)]
pub struct PartialByte {
    /// the partial byte
    pub(crate) byte: u8,

    /// the current position
    pub(crate) index: u8,
}

impl PartialByte {
    /// Checks if no partial byte has been read.
    ///
    /// Signifies full bytes can be read.
    pub fn is_on_byte_boundary(&self) -> bool {
        self.byte == 0 && self.index == 0
    }

    /// Initializes a [PartialByte] read and returns
    /// `n` MSB from `byte`.
    ///
    /// The bits will be aligned to the left.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut partial = PartialByte::default();
    /// let byte = partial.set_read(0b1011_1110, 5).unwrap();
    /// assert_eq!(byte, 0b1011_1000);
    /// ```
    ///
    /// # Errors
    ///
    /// 1. a previous `set_read` has not ended on a byte boundary
    /// 2. `n` is not \[1; 7\]
    pub fn set_read(&mut self, byte: u8, n: u8) -> Result<u8, PartialByteError> {
        snafu::ensure!(self.is_on_byte_boundary(), OverrideForbiddenSnafu);
        valid_n(n)?;

        self.byte = byte;

        self.read(n)
    }

    /// Reads `n` from the cached byte.
    ///
    /// The bits will be aligned to the left.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut partial = PartialByte::default();
    /// let byte = partial.set_read(0b1011_1110, 5).unwrap();
    /// assert_eq!(byte, 0b1011_1000);
    ///
    /// let byte = partial.read(3).unwrap();
    /// assert_eq!(byte, 0b1100_0000);
    /// ```
    ///
    /// # Errors
    ///
    /// 1. `n` is not \[1; 7\]
    pub fn read(&mut self, n: u8) -> Result<u8, PartialByteError> {
        valid_n(n)?;

        // create a mask to extract the bit range of interest
        // 0xFF right shift to get the number of bits required
        // shift that back to the left to move it to the starting
        // point of the bit range
        let mask = (0xFF >> (8 - n)) << (8 - (n + self.index));
        // partial byte AND mask extracts the bit range
        // left shift by index to left aligned the bit range
        let partial = (self.byte & mask) << self.index;

        // NAND the byte with mask to remove the extracted bits
        self.byte &= !mask;
        // increment index by n, MOD 8 for rollover
        self.index = (self.index + n) % 8;

        Ok(partial)
    }

    /// Write `n` bits of `byte`.
    ///
    /// Returns a fully written byte when
    /// a byte boundary is surpassed, otherwise
    /// returns None.
    pub fn write(&mut self, byte: u8, n: u8) -> Option<u8> {
        if n == 0 {
            return None;
        }

        // extract the first n bits
        let byte = (byte >> (8 - n)) << (8 - n);

        // append upper half of byte to the cached bits
        let full = self.byte + (byte >> self.index);

        // advance index
        let idx = self.index + n;
        self.index = idx % 8;

        if idx >= 8 {
            // byte boundary surpassed

            // lower half of byte becomes cached bits
            self.byte = byte << (n - self.index);
            // return full byte
            Some(full)
        } else {
            // no full byte written
            self.byte = full;
            None
        }
    }

    /// Returns the amount of remaining bits
    /// that can be read from the partial
    /// byte until the next byte boundary.
    pub fn remaining(&self) -> u8 {
        if self.is_on_byte_boundary() {
            // partial byte is uninitialized,
            // nothing can be read
            return 0;
        }
        8 - self.index
    }
}

impl Debug for PartialByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialByte")
            .field("byte", &format!("0b{:08b}", self.byte))
            .field("index", &self.index)
            .finish()
    }
}

/// Error when Partial Byte reading fails
#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum PartialByteError {
    /// tried to read invalid number of bits
    #[snafu(display("expected n of [1; 7], but got {n}"))]
    InvalidN { n: u8 },
    /// tried to overwrite cached bits
    #[snafu(display("tried to override initialized partial byte"))]
    OverrideForbidden,
}

/// Helper method to check if `n` is
/// between 1 and 7.
pub(super) fn valid_n(n: u8) -> Result<(), PartialByteError> {
    snafu::ensure!((1..8).contains(&n), InvalidNSnafu { n });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_on_byte_boundary_test() {
        let partial = PartialByte::default();
        assert!(partial.is_on_byte_boundary());

        let partial = PartialByte { byte: 5, index: 2 };
        assert!(!partial.is_on_byte_boundary());
    }

    #[test]
    fn set_read_test() {
        let mut partial = PartialByte::default();
        let byte = partial.set_read(0b1011_1110, 5);
        assert_eq!(byte, Ok(0b1011_1000));
        assert_eq!(
            partial,
            PartialByte {
                byte: 0b0000_0110,
                index: 5
            }
        );

        let invalid = partial.set_read(1, 1);
        assert_eq!(invalid, Err(PartialByteError::OverrideForbidden));

        let mut partial = PartialByte::default();
        let invalid = partial.set_read(1, 0);
        assert_eq!(invalid, Err(PartialByteError::InvalidN { n: 0 }));

        let mut partial = PartialByte::default();
        let invalid = partial.set_read(1, 9);
        assert_eq!(invalid, Err(PartialByteError::InvalidN { n: 9 }));
    }

    #[test]
    fn read_test() {
        let mut partial = PartialByte {
            byte: 0b0000_0110,
            index: 5,
        };
        let byte = partial.read(2);
        assert_eq!(byte, Ok(0b1100_0000));
        assert_eq!(
            partial,
            PartialByte {
                byte: 0b0000_0000,
                index: 7
            }
        );

        let mut partial = PartialByte {
            byte: 0b0000_0110,
            index: 5,
        };
        let byte = partial.read(3);
        assert_eq!(byte, Ok(0b1100_0000));
        assert_eq!(partial, PartialByte::default());

        let invalid = partial.read(0);
        assert_eq!(invalid, Err(PartialByteError::InvalidN { n: 0 }));

        let invalid = partial.read(9);
        assert_eq!(invalid, Err(PartialByteError::InvalidN { n: 9 }));
    }

    #[test]
    fn write_test() {
        let mut partial = PartialByte::default();
        let none = partial.write(0b1011_1110, 5);
        assert_eq!(none, None);
        assert_eq!(
            partial,
            PartialByte {
                byte: 0b1011_1000,
                index: 5
            }
        );

        let none = partial.write(0b1011_1110, 2);
        assert_eq!(none, None);
        assert_eq!(
            partial,
            PartialByte {
                byte: 0b1011_1100,
                index: 7
            }
        );

        let some = partial.write(0b1011_1110, 4);
        assert_eq!(some, Some(0b1011_1101));
        assert_eq!(
            partial,
            PartialByte {
                byte: 0b0110_0000,
                index: 3
            }
        );

        let some = partial.write(0b1011_1110, 5);
        assert_eq!(some, Some(0b0111_0111));
        assert_eq!(partial, PartialByte::default());

        let none = partial.write(1, 0);
        assert!(none.is_none());
    }

    #[test]
    fn remaining_test() {
        let mut partial = PartialByte::default();
        assert_eq!(partial.remaining(), 0);

        let _ = partial.set_read(5, 3);
        assert_eq!(partial.remaining(), 5);
    }

    #[test]
    fn valid_n_test() {
        for i in 1..=7 {
            let valid = valid_n(i);
            assert_eq!(valid, Ok(()))
        }

        let invalid = valid_n(0);
        assert_eq!(invalid, Err(PartialByteError::InvalidN { n: 0 }));

        let invalid = valid_n(9);
        assert_eq!(invalid, Err(PartialByteError::InvalidN { n: 9 }));
    }
}
