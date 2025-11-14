mod error;
mod varint;
mod varint_number;

use std::time::Duration;

use {
    crate::{VarIntNumber, bitstore::BitStore},
    funty::{AtMost32, Unsigned},
    snafu::ResultExt,
    std::str::FromStr,
    std::{
        fmt::{Debug, Display, LowerHex, UpperHex},
        hash::Hash,
    },
};

pub use error::{NumberConversion, NumberError};

pub(super) use error::{ctx, ctx_conv};

const MAX_U6: u64 = (2 << 5) - 1;
const MAX_U14: u64 = (2 << 13) - 1;
const MAX_U30: u64 = (2 << 29) - 1;
const MAX_U62: u64 = (2 << 61) - 1;

/// This is the quintessential
/// `VarInt` type of this crate. In the QUIC
/// and MOQT RFCs they are denoted by `x(i)`.
///
/// For convenience you should use the [x!(i)](varint_derive::x) // TODO doc.rs link once varint_derive is published
/// macro instead of remembering this type.
///
/// It has four possible sizes, identified
/// by the first 2 bits on the wire:
///
/// - `0b00`: next 6 bits are the number
/// - `0b01`: next 14 bits are the number
/// - `0b10`: next 30 bits are the number
/// - `0b11`: next 62 bits are the number
#[derive(Default, PartialEq, PartialOrd, Clone, Eq, Ord)]
pub struct Number {
    data: BitStore<6, 62>,
}

impl Number {
    /// Creates a new VarInt.
    ///
    /// This function only accept `u8`, `u16`
    /// and `u32` to provide an infallible
    /// constructor method.
    ///
    /// # Example
    ///
    /// ```
    /// # use varint_core::{Number, VarIntNumber};
    /// let v = Number::new(123u32);
    /// assert_eq!(v, 123);
    /// ```
    pub fn new<U>(v: U) -> Self
    where
        U: Unsigned + AtMost32,
    {
        let mut this = Self::default();
        this.set_number(v, None).expect("value will fit");
        this
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number::<u64>())
    }
}

impl LowerHex for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.number::<u64>(), f)
    }
}

impl UpperHex for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.number::<u64>(), f)
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.number::<u64>();
        f.debug_struct("VarInt")
            .field("value", &v)
            .field("num_bits", &super::num_bits(v))
            .field("inner", &self.data)
            .finish()
    }
}

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number::<u64>().hash(state);
    }
}

// partial eq with any primitive Numbers
macro_rules! impl_partial_eq {
    ( $($prim:ty),+ $(,)? ) => {
        // u64 is largest type Number can be
        // forcing
        $(
            impl PartialEq<$prim> for Number {
                fn eq(&self, other: &$prim) -> bool {
                    if *other < (0 as $prim) { return false; }
                    if (*other as u64) > MAX_U62 { return false; }
                    // now that it is verified that other is
                    // not negative or larger than allowed
                    // it is permissible to case both to u64
                    *other as u64 == self.number::<u64>()
                }
            }
            impl PartialEq<Number> for $prim {
                fn eq(&self, other: &Number) -> bool {
                    // flip the comparison to make use of
                    // the partial eq above
                    other == self
                }
            }
        )+
    };
}
impl_partial_eq!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

// partial ord with any primitive Numbers
macro_rules! impl_partial_ord    {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl PartialOrd<$prim> for Number {
                fn partial_cmp(&self, other: &$prim) -> Option<std::cmp::Ordering> {
                    if *other < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // other is positive -> casting to u64 is fine
                    let other = *other as u64;
                    if self.number::<u64>() > other {
                        Some(std::cmp::Ordering::Greater)
                    } else if self.number::<u64>() < other {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }
            }
            impl PartialOrd<Number> for $prim {
                fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
                    if *self < (0 as $prim) {
                        return Some(std::cmp::Ordering::Less);
                    }
                    // self is positive -> casting to u64 is fine
                    if (*self as u64) > other.number::<u64>() {
                        Some(std::cmp::Ordering::Greater)
                    } else if (*self as u64) < other.number::<u64>() {
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

// To and From Number and u8, u16, u32 (Unsigned + AtMost32)
macro_rules! impl_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl From<$prim> for Number {
                fn from(value: $prim) -> Self {
                    Self::new(value)
                }
            }
            impl From<&$prim> for Number {
                fn from(value: &$prim) -> Self {
                    Self::new(*value)
                }
            }
        )+
    };
}
impl_from!(u8, u16, u32);

// cast Number to u64 and u128
macro_rules! impl_from_number {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            impl From<Number> for $prim {
                fn from(value: Number) -> Self {
                    value.number()
                }
            }
        )+
    };
}
impl_from_number!(u64, u128);

// Try From all other primitives for Number
macro_rules! impl_try_from {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            // owned primitive
            impl TryFrom<$prim> for Number {
                type Error = NumberConversion<$prim>;
                fn try_from(value: $prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(value >= (0 as $prim), ctx_conv::IsNegativeSnafu { value });
                    // value is verified to be positive => casting
                    // to u64 is permissible
                    Number::new_number(value as u64, None).context(ctx_conv::InvalidSnafu { value })
                }
            }
            // ref primitive
            impl TryFrom<&$prim> for Number {
                type Error = NumberConversion<$prim>;
                fn try_from(value: &$prim) -> Result<Self, Self::Error> {
                    snafu::ensure!(*value >= (0 as $prim), ctx_conv::IsNegativeSnafu { value: *value });
                    // value is verified to be positive => casting
                    // to u64 is permissible
                    Number::new_number(*value as u64, None).context(ctx_conv::InvalidSnafu { value: *value })
                }
            }
        )+
    };
}
impl_try_from!(u64, u128, usize, i8, i16, i32, i64, i128, isize);

// Try From Number for primitives
macro_rules! impl_try_from_number {
    ( $($prim:ty),+ $(,)? ) => {
        $(
            // owned Number
            impl TryFrom<Number> for $prim {
                type Error = NumberConversion<$prim>;
                fn try_from(value: Number) -> Result<Self, Self::Error> {
                    snafu::ensure!(value <= <$prim>::MAX, ctx_conv::UnFitSnafu { value, max: <$prim>::MAX });

                    Ok(value.number::<u64>() as $prim)
                }
            }
            // ref Number
            impl TryFrom<&Number> for $prim {
                type Error = NumberConversion<$prim>;
                fn try_from(value: &Number) -> Result<Self, Self::Error> {
                    snafu::ensure!(*value <= <$prim>::MAX, ctx_conv::UnFitSnafu { value: value.clone(), max: <$prim>::MAX });

                    Ok(value.number::<u64>() as $prim)
                }
            }
        )+
    };
}
impl_try_from_number!(u8, u16, u32, usize, i8, i16, i32, i64, i128, isize);

impl FromStr for Number {
    type Err = NumberError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u64 = crate::number_from_str(s).context(ctx::StringSnafu)?;

        Self::new_number(num, None)
    }
}

impl From<Number> for Duration {
    fn from(value: Number) -> Self {
        Duration::from_millis(value.number())
    }
}

impl TryFrom<Duration> for Number {
    type Error = NumberError;
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new_number(value.as_millis(), None)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        pretty_assertions::{assert_eq, assert_ne},
    };

    #[test]
    fn new_test() {
        let valid = Number::new(15u8);
        assert_eq!(valid.number::<u8>(), 15u8);
        assert_eq!(valid.data.number::<u16>(), 15);

        let valid = Number::new(537u16);
        assert_eq!(valid.data.number::<u16>(), 537);

        let valid = Number::new(2_123_789u32);
        assert_eq!(valid.data.number::<u32>(), 2_123_789);
    }

    #[test]
    fn eq_test() {
        assert_eq!(Number::new(123u8), 123);
        assert_eq!(Number::new(537u16), 537);
        assert_eq!(Number::new(2_123_789u32), 2_123_789);

        assert_ne!(Number::new(123u8), -5);
        assert_ne!(Number::new(234u8), i128::MIN);
        assert_ne!(Number::new(123u16), i64::MAX);
    }

    #[test]
    fn ord_test() {
        let num = Number::new(123u8);
        assert!(num > 50u8);
        assert!(50u64 < num);
        assert!(num > 13i8);
        assert!(num < 250u16);
        assert!(250u16 > num);
        assert!(num == 123u64);
        assert!(123u8 == num);
    }

    #[test]
    fn from_test() {
        let num = Number::from(40u8);
        assert_eq!(num, 40);
        assert_eq!(u64::from(num.clone()), 40);
        assert_eq!(u64::from(num), 40);

        let num = Number::from(537u16);
        assert_eq!(num, 537);

        let num = Number::from(2_223_789_999u32);
        assert_eq!(num, 2_223_789_999u64);

        let invalid = i32::try_from(num.clone());
        assert_eq!(
            invalid,
            Err(NumberConversion::UnFit {
                value: num,
                max: i32::MAX
            })
        );
    }

    #[test]
    fn try_from_test() {
        let Ok(valid) = Number::try_from(9_000_000_000u64) else {
            unreachable!("is valid u62 number")
        };
        assert_eq!(valid, 9_000_000_000u64);

        let Ok(valid) = Number::try_from(MAX_U62) else {
            unreachable!("is valid u62 number")
        };
        assert_eq!(valid, MAX_U62);

        let invalid = Number::try_from(MAX_U62 + 1);
        assert_eq!(
            invalid,
            Err(NumberConversion::Invalid {
                value: MAX_U62 + 1,
                source: NumberError::TooLarge {
                    num: MAX_U62 as u128 + 1
                }
            })
        );

        let invalid = Number::try_from(-1);
        assert_eq!(invalid, Err(NumberConversion::IsNegative { value: -1 }));

        let num = Number::new(537u16);
        let invalid = u8::try_from(num.clone());
        assert_eq!(
            invalid,
            Err(NumberConversion::UnFit {
                value: num,
                max: u8::MAX
            })
        );

        let num = Number::new(2_223_789_999u32);
        let invalid = i32::try_from(num.clone());
        assert_eq!(
            invalid,
            Err(NumberConversion::UnFit {
                value: num,
                max: i32::MAX
            })
        );
    }
}
