#[cfg(feature = "moq")]
use crate::BinaryData;

#[cfg(feature = "moq")]
#[derive(Debug, Default)]
pub struct Tuple {
    data: Vec<BinaryData>,
}
