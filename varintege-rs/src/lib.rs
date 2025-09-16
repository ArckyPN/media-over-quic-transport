mod x;

#[cfg(feature = "moq")]
pub use varint_core::{BinaryData, Tuple};
pub use varint_core::{BitNumber, BitRange, Number};

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
}
