use snafu::Snafu;

#[derive(Debug, Default, PartialEq)]
pub struct PartialByteR {
    /// the partial byte
    pub(crate) byte: Option<u8>,

    /// the current position
    pub(crate) index: u8,
}

impl PartialByteR {
    /// Checks if no partial byte has been read.
    ///
    /// Signifies full bytes can be read.
    pub fn is_on_byte_boundary(&self) -> bool {
        self.byte.is_none() && self.index == 0
    }

    /// Initializes a [PartialByte] read and returns `n`
    /// MSB from `byte`.
    ///
    /// Saves the remaining partial byte for the next
    /// read.
    pub fn set(&mut self, n: u8, byte: u8) -> Result<u8, PartialByteError> {
        snafu::ensure!(self.is_on_byte_boundary(), OverrideForbiddenSnafu);
        valid_n(n)?;

        self.byte = Some(byte);

        self.get(n)
    }

    /// Returns `n` bits from the initialized
    /// partial byte.
    pub fn get(&mut self, n: u8) -> Result<u8, PartialByteError> {
        valid_n(n)?;
        let Some(byte) = self.byte else {
            return Err(PartialByteError::NotInitialized);
        };

        let shift = 8 - (n + self.index);
        let partial = byte >> shift;

        self.index = (self.index + n) % 8;
        self.byte = if shift == 0 {
            None
        } else {
            Some(Self::remove_bits(byte, self.index)?)
        };

        if self.index == 0 {
            assert!(
                self.byte.is_none(),
                "when index rolls over to 0, byte should be None"
            )
        }

        Ok(partial)
    }

    /// Returns the amount of remaining bits
    /// that can be read from the partial
    /// byte.
    pub fn remaining(&self) -> u8 {
        if self.is_on_byte_boundary() {
            return 0;
        }
        8 - self.index
    }

    // TODO doc
    pub fn remove_bits(byte: u8, shift: u8) -> Result<u8, PartialByteError> {
        valid_n(shift)?;
        Ok((0xFF >> shift) & byte)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct PartialByteW {
    /// the partial byte
    pub(crate) byte: u8,

    /// the current position
    pub(crate) index: u8,
}

impl PartialByteW {
    /// Checks if no partial byte has been read.
    ///
    /// Signifies full bytes can be read.
    pub fn is_on_byte_boundary(&self) -> bool {
        self.index == 0
    }

    /// Returns the full byte when it is full.
    ///
    /// When not full, returns None.
    pub fn set(&mut self, n: u8, byte: u8) -> Option<u8> {
        let full = if self.index != 0 {
            self.byte + (byte >> self.index)
        } else {
            byte
        };

        let idx = self.index + n;
        self.index = idx % 8;

        if idx >= 8 {
            self.byte = if self.index != 0 {
                byte << (8 - self.index)
            } else {
                0
            };

            Some(full)
        } else {
            self.byte = full;
            None
        }
    }
}

#[derive(Debug, Snafu, PartialEq, Clone)]
pub enum PartialByteError {
    #[snafu(display("tried to read partial byte with none initialized"))]
    NotInitialized,
    #[snafu(display("tried to read >{got}< bits from >{has}< bits buffer"))]
    OutOfBoundary { got: u8, has: u8 },
    #[snafu(display("expected n of [1; 7], but got >{n}<"))]
    InvalidN { n: u8 },
    #[snafu(display("tried to override initialized partial byte"))]
    OverrideForbidden,
}

/// Helper method to check if `n` is
/// between 1 and 7.
pub(super) fn valid_n(n: u8) -> Result<(), PartialByteError> {
    snafu::ensure!((1..8).contains(&n), InvalidNSnafu { n });
    Ok(())
}
