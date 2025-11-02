use funty::Unsigned;

/// Returns the number of bits required to
/// stored `v`.
pub fn num_bits<U>(v: U) -> usize
where
    U: Unsigned,
{
    let ones = v.count_ones();
    if ones == 0 {
        return 1;
    }
    (v.count_ones() + v.count_zeros() - v.leading_zeros()) as usize
}

/// Shifts `len` bits of `buf` to the left to align
/// them to the left.
pub fn shift_bits(buf: &mut [u8], len: usize) {
    let n = 8 - (len % 8);

    // no shift required when already on a byte boundary
    if n == 8 {
        return;
    }

    let len = buf.len();

    // shift full buffer by n to the left
    for i in 0..len {
        // shift lower half to he left
        buf[i] <<= n;

        // skip last step for final byte
        if i == len - 1 {
            continue;
        }

        // add upper half of the next byte to current byte
        buf[i] += buf[i + 1] >> (8 - n);
    }
}

/// Shifts `len` bits of `buf` to the right to
/// reverse the alignment from `shift_bits`.
pub fn unshift_bits(buf: &mut [u8], len: usize) {
    let n = 8 - (len % 8);

    // no shift required when already on a byte boundary
    if n == 8 {
        return;
    }

    let len = buf.len();

    // shift full buffer by n to the right
    for i in (0..len).rev() {
        // shift upper half to the right
        buf[i] >>= n;

        // skip last step for the first byte
        if i == 0 {
            continue;
        }

        // add lower half of previous byte to current byte
        buf[i] += buf[i - 1] << (8 - n);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn num_bits_test() {
        let bits = num_bits(0b1100_1111_u8);
        assert_eq!(bits, 8);

        let bits = num_bits(u16::MAX);
        assert_eq!(bits, 16);

        let bits = num_bits(0_u64);
        assert_eq!(bits, 1);

        let bits = num_bits(0b0010_0000_u64);
        assert_eq!(bits, 6);
    }

    #[test]
    fn shift_bits_test() {
        let mut buf = vec![0b0000_1010];
        shift_bits(&mut buf, 5);
        assert_eq!(buf, &[0b0101_0000]);

        let mut buf = vec![0b0000_1010];
        shift_bits(&mut buf, 8);
        assert_eq!(buf, &[0b0000_1010]);

        let mut buf = vec![0, 0b0110_1010];
        shift_bits(&mut buf, 13);
        assert_eq!(buf, &[0b0000_0011, 0b0101_0000]);
    }

    #[test]
    fn unshift_bits_test() {
        let mut buf = vec![0b0101_0000];
        unshift_bits(&mut buf, 5);
        assert_eq!(buf, &[0b0000_1010]);

        let mut buf = vec![0b0000_1010];
        unshift_bits(&mut buf, 8);
        assert_eq!(buf, &[0b0000_1010]);

        let mut buf = vec![0b0000_0011, 0b0101_0000];
        unshift_bits(&mut buf, 13);
        assert_eq!(buf, &[0, 0b0110_1010]);
    }
}
