mod error;
mod varint;
mod varint_bytes;
mod varint_number;

use std::fmt::Debug;

use {
    crate::{VarIntBytes, VarIntNumber, bitstore::BitStore},
    snafu::ResultExt,
};

pub use error::{BitRangeConversion, BitRangeError};

pub(super) use error::{ctx, ctx_conv};

// TODO in the proc macro make sure this ends on a bytes boundary, current num bits % 8 == 0
/// This type represents the type `x (A..B)`
/// in the QUIC RFC.
///
/// It is a Number that has a binary
/// representation with a number of
/// bits between `A` and `B`. It always
/// ends on a byte boundary.
///
/// `A` and `B` can both be omitted to
/// open their respective range limits.
#[derive(Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BitRange<const MIN: usize = 0, const MAX: usize = { usize::MAX }> {
    data: BitStore<MIN, MAX>,
}

impl<const MIN: usize, const MAX: usize> Debug for BitRange<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitRange")
            .field("min_bits", &MIN)
            .field("max_bits", &MAX)
            .field("inner", &self.data)
            .finish()
    }
}

macro_rules! impl_partial_eq {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const MIN: usize, const MAX: usize> PartialEq<$prim> for BitRange<MIN, MAX> {
                fn eq(&self, other: &$prim) -> bool {
                    if *other < (0 as $prim) { return false; }
                    // now that it is verified that other is
                    // not negative or larger than allowed
                    // it is permissible to case both to u64
                    *other as u128 == self.number::<u128>()
                }
            }
            impl<const MIN: usize, const MAX: usize> PartialEq<BitRange<MIN, MAX>> for $prim {
                fn eq(&self, other: &BitRange<MIN, MAX>) -> bool {
                    other == self
                }
            }
        )+
    };
}
impl_partial_eq!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_partial_ord    {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const MIN: usize, const MAX: usize> PartialOrd<$prim> for BitRange<MIN, MAX> {
                fn partial_cmp(&self, other: &$prim) -> Option<std::cmp::Ordering> {
                    if *other < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // other is positive -> casting to u128 is fine
                    let other = *other as u128;
                    if self.number::<u128>() > other {
                        Some(std::cmp::Ordering::Greater)
                    } else if self.number::<u128>() < other {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }
            }
            impl<const MIN: usize, const MAX: usize> PartialOrd<BitRange<MIN, MAX>> for $prim {
                fn partial_cmp(&self, other: &BitRange<MIN, MAX>) -> Option<std::cmp::Ordering> {
                    if *self < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // self is positive -> casting to u128 is fine
                    if (*self as u128) > other.number::<u128>() {
                        Some(std::cmp::Ordering::Greater)
                    } else if (*self as u128) < other.number::<u128>() {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }
            }
        )+
    };
}
impl_partial_ord!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_try_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const MIN: usize, const MAX: usize> TryFrom<$prim> for BitRange<MIN, MAX> {
                type Error = BitRangeConversion<$prim>;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(value >= (0 as $prim), ctx_conv::IsNegativeSnafu { value });
                    // value is verified to be positive => casting
                    // to u128 is permissible
                    Self::new_number(value as u128, None).context(ctx_conv::InvalidSnafu { value })
                }
            }
            impl<const MIN: usize, const MAX: usize> TryFrom<BitRange<MIN, MAX>> for $prim {
                type Error = BitRangeConversion<$prim>;
                fn try_from(value: BitRange<MIN, MAX>) -> Result<Self, Self::Error> {
                    snafu::ensure!(value <= <$prim>::MAX, ctx_conv::UnFitSnafu { value: value.number::<u128>(), max: <$prim>::MAX });

                    Ok(value.number::<u128>() as $prim)
                }
            }
        )+
    };
}
impl_try_from!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl PartialEq<&str> for BitRange {
    fn eq(&self, other: &&str) -> bool {
        self.bytes() == other.as_bytes()
    }
}

impl PartialEq<String> for BitRange {
    fn eq(&self, other: &String) -> bool {
        self.bytes() == other.as_bytes()
    }
}

impl PartialEq<&[u8]> for BitRange {
    fn eq(&self, other: &&[u8]) -> bool {
        self.bytes() == other
    }
}

impl From<&str> for BitRange {
    fn from(value: &str) -> Self {
        Self::new_bytes(value.as_bytes(), Some(value.len() * 8)).expect("will fit")
    }
}

impl From<String> for BitRange {
    fn from(value: String) -> Self {
        Self::new_bytes(value.as_bytes(), Some(value.len() * 8)).expect("will fit")
    }
}

impl From<&[u8]> for BitRange {
    fn from(value: &[u8]) -> Self {
        Self::new_bytes(value, Some(value.as_ref().len() * 8)).expect("will fit")
    }
}

impl<const N: usize> From<[u8; N]> for BitRange {
    fn from(value: [u8; N]) -> Self {
        Self::new_bytes(&value, Some(N * 8)).expect("will fit")
    }
}

impl<const N: usize> From<&[u8; N]> for BitRange {
    fn from(value: &[u8; N]) -> Self {
        Self::new_bytes(value, Some(N * 8)).expect("will fit")
    }
}

#[cfg(test)]
mod tests {
    use {super::*, pretty_assertions::assert_eq};

    #[test]
    fn from_buf_test() {
        const BUF: &[u8] = &[1, 2, 3];

        let valid = BitRange::from(BUF);
        assert_eq!(valid, BUF);
    }

    #[test]
    fn from_str_test() {
        const STR_SLICE: &str = "message";
        let static_str: &'static str = "data";
        let owned_string = "my stuff".to_owned();

        let valid = BitRange::from(STR_SLICE);
        assert_eq!(valid, STR_SLICE);

        let valid = BitRange::from(static_str);
        assert_eq!(valid, static_str);

        let valid = BitRange::from(owned_string.clone());
        assert_eq!(valid, owned_string);
    }

    #[test]
    fn eq_test() {
        assert_eq!(
            BitRange::<0>::new_number(16u8, None).expect("will fit"),
            16u32
        );
        assert_eq!(
            BitRange::<0>::new_number(100u16, None).expect("will fit"),
            100i32
        );
        assert_eq!(BitRange::<0>::default(), 0);

        assert_ne!(
            BitRange::<8, 16>::new_number(100u128, None).expect("will fit"),
            -5
        );
        assert_ne!(BitRange::<8, 16>::default(), 100);
        assert_ne!(
            BitRange::<8, 16>::new_number(100u128, None).expect("will fit"),
            1
        );
    }

    #[test]
    fn ord_test() {
        let num: BitRange<8, 16> = BitRange::new_number(123u8, None).expect("will fit");
        assert!(num > 50u8);
        assert!(50u64 < num);
        assert!(num > 13i8);
        assert!(num < 250u16);
        assert!(250u16 > num);
        assert!(num == 123u64);
        assert!(123u8 == num);
    }

    #[test]
    fn try_from_test() {
        let valid: BitRange<8, 16> = BitRange::try_from(5_123i16).expect("will fit");
        assert_eq!(valid, 5_123u16);

        let invalid = BitRange::<0, 2>::try_from(100);
        assert_eq!(
            invalid,
            Err(BitRangeConversion::Invalid {
                value: 100,
                source: BitRangeError::InvalidCapacity {
                    value: 100,
                    needs: 7,
                    cap: 2
                }
            })
        );

        let invalid = BitRange::<8, 16>::try_from(-5);
        assert_eq!(invalid, Err(BitRangeConversion::IsNegative { value: -5 }));

        assert_eq!(u16::try_from(valid.clone()).expect("will fit"), 5_123);
        assert_eq!(
            u8::try_from(valid),
            Err(BitRangeConversion::UnFit {
                value: 5_123,
                max: u8::MAX
            })
        );
    }
}
