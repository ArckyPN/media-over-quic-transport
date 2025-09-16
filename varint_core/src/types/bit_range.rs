use bitvec::prelude::*;

#[derive(Debug, Default)]
pub struct BitRange<const MIN: usize = 0, const MAX: usize = { usize::MAX }> {
    data: BitVec<u8>,
}
