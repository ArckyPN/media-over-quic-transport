use bitvec::prelude::*;

// BitNumber<N> => x (N) | N bits length with any value acceptable
#[derive(Debug, Default)]
pub struct BitNumber<const N: usize, const MIN: usize = 0, const MAX: usize = { usize::MAX }> {
    data: BitVec<u8>,
}
