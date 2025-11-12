use {
    super::{MAX_U6, MAX_U14, MAX_U30, MAX_U62, Number, ctx},
    crate::VarIntNumber,
    snafu::ResultExt,
};

impl VarIntNumber for Number {
    fn number<U>(&self) -> U
    where
        U: funty::Unsigned,
    {
        self.data.number()
    }

    fn set_number<U>(&mut self, v: U, _n: Option<usize>) -> Result<&mut Self, Self::Error>
    where
        U: funty::Unsigned,
    {
        snafu::ensure!(
            v.as_u128() <= (MAX_U62 as u128),
            ctx::TooLargeSnafu { num: v.as_u128() }
        );

        let len = match v {
            x if x.as_u64() <= MAX_U6 => 6,
            x if x.as_u64() <= MAX_U14 => 14,
            x if x.as_u64() <= MAX_U30 => 30,
            x if x.as_u64() <= MAX_U62 => 62,
            _ => unreachable!("number cannot be larger than (2 << 61) - 1"),
        };
        self.data
            .set_number(v, Some(len))
            .context(ctx::BitStoreSnafu)?;

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::NumberError,
        pretty_assertions::{assert_eq, assert_ne},
    };

    #[test]
    fn new_number_test() {
        let valid = Number::new_number(15u8, None);
        assert_eq!(valid, Ok(Number::new(15u16)));

        let valid = Number::new_number(9_000_000_000u64, None);
        assert_eq!(valid.map(|n| n.number::<u64>()), Ok(9_000_000_000u64));

        let valid = Number::new_number(MAX_U62, None);
        assert_eq!(valid.map(|n| n.number::<u64>()), Ok(MAX_U62));

        let valid = Number::new_number(MAX_U62 + 1, None);
        assert_eq!(
            valid,
            Err(NumberError::TooLarge {
                num: MAX_U62 as u128 + 1
            })
        );
    }

    #[test]
    fn set_number_test() {
        let mut base = Number::default();

        let valid = base.set_number(8u8, None);
        assert!(valid.is_ok());
        assert_eq!(base, 8);

        let valid = base.set_number(700u16, None);
        assert!(valid.is_ok());
        assert_eq!(base, 700);

        let valid = base.set_number(2_123_789u32, None);
        assert!(valid.is_ok());
        assert_eq!(base, 2_123_789);

        let valid = base.set_number(MAX_U62, None);
        assert!(valid.is_ok());
        assert_eq!(base, MAX_U62);

        let err = base.set_number(MAX_U62 + 1, None);
        assert_eq!(
            err,
            Err(NumberError::TooLarge {
                num: MAX_U62 as u128 + 1
            })
        );
    }

    #[test]
    fn number_test() {
        let valid = Number::new(15u8);
        assert_eq!(valid.number::<u8>(), 15);

        let valid = Number::new(789u32);
        assert_eq!(valid.number::<u16>(), 789);

        let valid = Number::new(u32::MAX);
        assert_eq!(valid.number::<u64>(), u32::MAX as u64);

        let valid = Number::new(0u8);
        assert_ne!(valid.number::<u8>(), 15);
    }
}
