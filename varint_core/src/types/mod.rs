#[cfg(feature = "moq")]
mod binary_data;
mod bit_number;
mod bit_range;
mod number;
#[cfg(feature = "moq")]
mod tuple;

#[cfg(feature = "moq")]
pub use {
    binary_data::{BinaryData, BinaryDataError},
    tuple::{Tuple, TupleError},
};
pub use {
    bit_number::{BitNumber, BitNumberConversion, BitNumberError},
    bit_range::{BitRange, BitRangeConversion, BitRangeError},
    number::{Number, NumberConversion, NumberError},
};

/// Returns the number of bits required to
/// stored `v`.
pub(super) fn num_bits<U>(v: U) -> usize
where
    U: funty::Unsigned,
{
    let ones = v.count_ones();
    if ones == 0 {
        return 1;
    }
    (v.count_ones() + v.count_zeros() - v.leading_zeros()) as usize
}

#[cfg(test)]
mod tests {
    use {super::*, pretty_assertions::assert_eq};

    #[test]
    fn num_bits_test() {
        assert_eq!(num_bits(5u8), 3);
        assert_eq!(num_bits(24u8), 5);
        assert_eq!(num_bits(86234285u32), 27);
        assert_eq!(num_bits(862342859999999999u64), 60);
        assert_eq!(num_bits(0u8), 1);
    }
}
