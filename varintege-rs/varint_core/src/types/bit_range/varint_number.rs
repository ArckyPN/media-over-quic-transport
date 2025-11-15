use {
    super::{BitRange, ctx},
    crate::{VarIntNumber, types::num_bits},
    snafu::ResultExt,
};

impl<const MIN: usize, const MAX: usize> VarIntNumber for BitRange<MIN, MAX> {
    fn set_number<U>(&mut self, v: U, n: Option<usize>) -> Result<&mut Self, Self::Error>
    where
        U: funty::Unsigned,
    {
        let num_bits = num_bits(v);
        let len = match n {
            Some(bits) => {
                if bits < MIN {
                    MIN
                } else {
                    bits
                }
            }
            None => num_bits,
        };

        // TODO must length be larger than MIN?

        // ensure v fits
        snafu::ensure!(
            len <= MAX,
            ctx::InvalidCapacitySnafu {
                value: v.as_u128(),
                needs: len,
                cap: MAX,
            }
        );

        let len = if len < MIN { MIN } else { len };

        self.data
            .set_number(v, Some(len))
            .context(ctx::BitStoreSnafu)?;

        Ok(self)
    }

    fn number<U>(&self) -> U
    where
        U: funty::Unsigned,
    {
        self.data.number()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::BitRangeError, pretty_assertions::assert_eq};

    #[test]
    fn new_number_test() {
        let valid = BitRange::<8, 16>::new_number(123u16, None);
        assert_eq!(valid.map(|n| n.number::<u8>()), Ok(123));

        let invalid = BitRange::<8, 16>::new_number(u32::MAX, None);
        assert_eq!(
            invalid,
            Err(BitRangeError::InvalidCapacity {
                value: u32::MAX as u128,
                needs: 32,
                cap: 16
            })
        );
    }

    #[test]
    fn set_number_test() {
        let mut base = BitRange::<8, 16>::default();

        let valid = base.set_number(8u8, Some(8));
        assert!(valid.is_ok());
        assert_eq!(base, 8);

        let invalid = base.set_number(u32::MAX, Some(32));
        assert_eq!(
            invalid,
            Err(BitRangeError::InvalidCapacity {
                value: u32::MAX as u128,
                needs: 32,
                cap: 16
            })
        );
    }

    #[test]
    fn number_test() {
        let valid = BitRange::<8, 16>::new_number(18u8, None).expect("will fit");
        assert_eq!(valid.number::<u8>(), 18);
    }
}
