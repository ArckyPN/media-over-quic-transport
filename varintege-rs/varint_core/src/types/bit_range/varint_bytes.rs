use {
    super::{BitRange, ctx},
    crate::VarIntBytes,
    snafu::ResultExt,
};

impl<const MIN: usize, const MAX: usize> VarIntBytes for BitRange<MIN, MAX> {
    fn bytes(&self) -> bytes::Bytes {
        self.data.bits()
    }

    fn new_bytes(buf: &[u8], n: Option<usize>) -> Result<Self, Self::Error> {
        let mut this = Self::default();
        this.set_bytes(buf, n)?;
        Ok(this)
    }

    fn set_bytes(&mut self, buf: &[u8], n: Option<usize>) -> Result<&mut Self, Self::Error> {
        let n = n.unwrap_or(buf.len() * 8);
        self.data.set_bits(buf, n).context(ctx::BitStoreSnafu)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::VarIntNumber, pretty_assertions::assert_eq};

    #[test]
    fn new_bytes_test() {
        let valid =
            BitRange::<8, 16>::new_bytes(&[0b1100_0011, 0b0011_0000], Some(12)).expect("will fit");
        assert_eq!(*valid.bytes(), [0b1100_0011, 0b0011_0000]);
    }

    #[test]
    fn bytes_test() {
        let num = BitRange::<8, 64>::new_number(u32::MAX as u64 + 1, None).expect("will fit");

        assert_eq!(*num.bytes(), [128, 0, 0, 0, 0]);
        assert_eq!(num, u32::MAX as u64 + 1)
    }

    #[test]
    fn set_bytes_test() {
        let mut num = BitRange::<8, 16>::default();
        num.set_bytes([1u8, 0].as_slice(), Some(9))
            .expect("will fit");

        assert_eq!(num, 2);
        assert_eq!(*num.bytes(), [1, 0]);
    }
}
