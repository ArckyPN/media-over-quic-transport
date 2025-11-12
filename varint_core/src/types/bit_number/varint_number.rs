use {
    super::{BitNumber, ctx},
    crate::{VarIntNumber, types::num_bits},
    snafu::ResultExt,
};

impl<const N: usize, const MIN: u128, const MAX: u128> VarIntNumber for BitNumber<N, MIN, MAX> {
    fn set_number<U>(&mut self, v: U, _n: Option<usize>) -> Result<&mut Self, Self::Error>
    where
        U: funty::Unsigned,
    {
        let len = num_bits(v);

        // ensure v fits
        snafu::ensure!(
            len <= N,
            ctx::InvalidCapacitySnafu {
                value: v.as_u128(),
                needs: len,
                cap: N
            }
        );
        snafu::ensure!(
            v.as_u128() >= MIN && v.as_u128() <= MAX,
            ctx::OutOfRangeSnafu {
                value: v.as_u128(),
                min: MIN,
                max: MAX
            }
        );

        self.data
            .set_number(v, Some(N))
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
    use {super::*, crate::BitNumberError, pretty_assertions::assert_eq};

    #[test]
    fn new_number_test() {
        let valid = BitNumber::<16>::new_number(15u8, None);
        assert_eq!(valid.map(|n| n.data.number::<u16>()), Ok(15));

        let invalid = BitNumber::<16>::new_number(u32::MAX, None);
        assert_eq!(
            invalid,
            Err(BitNumberError::InvalidCapacity {
                value: u32::MAX as u128,
                needs: 32,
                cap: 16
            })
        );

        let valid = BitNumber::<8, 5>::new_number(100u8, None);
        assert_eq!(valid.map(|n| n.data.number::<u16>()), Ok(100));

        let invalid = BitNumber::<8, 5>::new_number(3u8, None);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 3,
                min: 5,
                max: u128::MAX
            })
        );

        let valid = BitNumber::<8, 0, 20>::new_number(20u8, None);
        assert_eq!(valid.map(|n| n.data.number::<u16>()), Ok(20));

        let invalid = BitNumber::<8, 0, 20>::new_number(21u8, None);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 21,
                min: 0,
                max: 20
            })
        );
    }

    #[test]
    fn set_number_test() {
        let mut base = BitNumber::<16>::default();

        let valid = base.set_number(15u8, None);
        assert!(valid.is_ok());
        assert_eq!(base, 15);

        // number needs more bits than available
        let invalid = base.set_number(u32::MAX, None);
        assert_eq!(
            invalid,
            Err(BitNumberError::InvalidCapacity {
                value: u32::MAX as u128,
                needs: 32,
                cap: 16
            })
        );

        let mut base = BitNumber::<8, 5>::default();

        let valid = base.set_number(100u8, None);
        assert!(valid.is_ok());
        assert_eq!(base, 100);

        // below the minimum
        let invalid = base.set_number(3u8, None);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 3,
                min: 5,
                max: u128::MAX
            })
        );

        let mut base = BitNumber::<8, 0, 20>::default();

        let valid = base.set_number(20u16, None);
        assert!(valid.is_ok());
        assert_eq!(base, 20);

        // above the maximum
        let invalid = base.set_number(21u8, None);
        assert_eq!(
            invalid,
            Err(BitNumberError::OutOfRange {
                value: 21,
                min: 0,
                max: 20
            })
        );
    }

    #[test]
    fn number_test() {
        let valid = BitNumber::<32>::new_number(45u16, None).expect("will fit");
        assert_eq!(valid.number::<u8>(), 45);
    }
}
