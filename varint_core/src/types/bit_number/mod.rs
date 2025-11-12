mod error;
mod varint;
mod varint_number;

use {
    crate::{VarIntNumber, bitstore::BitStore},
    snafu::ResultExt,
    std::fmt::{Debug, Display},
};

pub use error::{BitNumberConversion, BitNumberError};

pub(super) use error::{ctx, ctx_conv};

/// This type represents several types of
/// the QUIC RFC:
///
/// - `x (N)` -> `BitNumber<N>` a Number
///   represented by `N` bits
///     - For convenience use: [x!(N)](varint_derive::x) // TODO doc.rs link once varint_derive is published
/// - `x (N) = C` -> `BitNumber<N, C, C>`
///   a Number represented by `N` Bits
///   with the const value `C`
///     - For convenience use: [x!(N = C)](varint_derive::x) // TODO doc.rs link once varint_derive is published
/// - `x (N) = C..D` -> `BitNumber<N, C, D>`
///   a Number represented by `N` Bits
///   with a value between `C` and `D`
///   (inclusive), C and D are optional
///     - For convenience use: [x!(N = C..D)](varint_derive::x) // TODO doc.rs link once varint_derive is published
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct BitNumber<const N: usize, const MIN: u128 = 0, const MAX: u128 = { u128::MAX }> {
    data: BitStore<N, N>,
}

// Display
impl<const N: usize, const MIN: u128, const MAX: u128> Display for BitNumber<N, MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.number::<u128>().to_string())
    }
}

// Debug
impl<const N: usize, const MIN: u128, const MAX: u128> Debug for BitNumber<N, MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitNumber")
            .field("value", &self.number::<u128>())
            .field("num_bits", &N)
            .field("min_value", &MIN)
            .field("max_value", &MAX)
            .field("inner", &self.data)
            .finish()
    }
}

// partial eq with all primitives
macro_rules! impl_partial_eq {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialEq<$prim> for BitNumber<N, MIN, MAX> {
                fn eq(&self, other: &$prim) -> bool {
                    if *other < (0 as $prim) { return false; }
                    // now that it is verified that other is
                    // not negative or larger than allowed
                    // it is permissible to case both to u64
                    *other as u128 == self.number::<u128>()
                }
            }
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialEq<BitNumber<N, MIN, MAX>> for $prim {
                fn eq(&self, other: &BitNumber<N, MIN, MAX>) -> bool {
                    other == self
                }
            }
        )+
    };
}
impl_partial_eq!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

// partial ord with all primitives
macro_rules! impl_partial_ord    {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialOrd<$prim> for BitNumber<N, MIN, MAX> {
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
            impl<const N: usize, const MIN: u128, const MAX: u128> PartialOrd<BitNumber<N, MIN, MAX>> for $prim {
                fn partial_cmp(&self, other: &BitNumber<N, MIN, MAX>) -> Option<std::cmp::Ordering> {
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

// try from/into for all primitives
macro_rules! impl_try_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl<const N: usize, const MIN: u128, const MAX: u128> TryFrom<$prim> for BitNumber<N, MIN, MAX> {
                type Error = BitNumberConversion<$prim>;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(value >= (0 as $prim), ctx_conv::IsNegativeSnafu { value });
                    // value is verified to be positive => casting
                    // to u128 is permissible
                    Self::new_number(value as u128, None).context(ctx_conv::InvalidSnafu { value })
                }
            }
            impl<const N: usize, const MIN: u128, const MAX: u128> TryFrom<BitNumber<N, MIN, MAX>> for $prim {
                type Error = BitNumberConversion<$prim>;
                fn try_from(value: BitNumber<N, MIN, MAX>) -> Result<Self, Self::Error> {
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        pretty_assertions::{assert_eq, assert_ne},
    };

    #[test]
    fn eq_test() {
        assert_eq!(
            BitNumber::<8>::new_number(100u8, None).expect("will fit"),
            100u16
        );
        assert_eq!(
            BitNumber::<16, 10>::new_number(10u16, None).expect("will fit"),
            10i16
        );
        assert_eq!(
            BitNumber::<8, 0, 100>::new_number(100u8, None).expect("will fit"),
            100u16
        );

        assert_ne!(
            BitNumber::<8>::new_number(100u8, None).expect("will fit"),
            101i128
        );
        assert_ne!(
            BitNumber::<16, 10>::new_number(10u16, None).expect("will fit"),
            -5
        );
        assert_ne!(
            BitNumber::<32, 0, 1>::new_number(1u128, None).expect("will fit"),
            100u16
        );
    }

    #[test]
    fn ord_test() {
        let num = BitNumber::<32, 5, 1_000>::new_number(123u128, None).expect("will fit");
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
        let valid = BitNumber::<16>::try_from(5_123u16).expect("will fit");
        assert_eq!(valid, 5_123u64);

        let valid = BitNumber::<16, 5>::try_from(5i8).expect("will fit");
        assert_eq!(valid, 5u8);

        let invalid = BitNumber::<8, 0, 10>::try_from(11);
        assert_eq!(
            invalid,
            Err(BitNumberConversion::Invalid {
                value: 11,
                source: BitNumberError::OutOfRange {
                    value: 11,
                    min: 0,
                    max: 10
                }
            })
        );

        let invalid = BitNumber::<8>::try_from(-1);
        assert_eq!(invalid, Err(BitNumberConversion::IsNegative { value: -1 }));

        let invalid = BitNumber::<1>::try_from(2u128);
        assert_eq!(
            invalid,
            Err(BitNumberConversion::Invalid {
                value: 2,
                source: BitNumberError::InvalidCapacity {
                    value: 2,
                    needs: 2,
                    cap: 1
                }
            })
        );

        let num: BitNumber<16> = BitNumber::try_from(5_123u16).expect("will fit");
        assert_eq!(u16::try_from(num.clone()).expect("will fit"), 5_123);
        assert_eq!(
            u8::try_from(num),
            Err(BitNumberConversion::UnFit {
                value: 5_123,
                max: u8::MAX
            })
        );
    }
}
