#[cfg(feature = "moq")]
mod binary_data;
pub mod bit_number;
mod bit_range;
mod number;
#[cfg(feature = "moq")]
mod tuple;

#[cfg(feature = "moq")]
pub use binary_data::BinaryData;
pub use bit_number::{BitNumber, BitNumberError};
pub use bit_range::{BitRange, BitRangeNumberError};
pub use number::{Number, NumberError};
#[cfg(feature = "moq")]
pub use tuple::Tuple;

use funty::Unsigned;

/// Returns the number of bits required to
/// stored `v`.
pub(super) fn num_bits<U>(v: U) -> usize
where
    U: Unsigned,
{
    let ones = v.count_ones();
    if ones == 0 {
        return 1;
    }
    (v.count_ones() + v.count_zeros() - v.leading_zeros()) as usize
}

// TODO impl standard Number stuff on Number, BitNumber and BitRange (add, sub, mult, etc.)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_bits_test() {
        assert_eq!(num_bits(5u8), 3);
        assert_eq!(num_bits(24u8), 5);
        assert_eq!(num_bits(86234285u32), 27);
        assert_eq!(num_bits(862342859999999999u64), 60);
        assert_eq!(num_bits(0u8), 1);
    }

    #[test]
    fn decode_test() {
        // TODO decoding a mix of all types
    }

    #[test]
    fn encode_test() {
        // TODO encoding a mix of all types
    }
}
