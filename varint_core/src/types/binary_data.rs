use bytes::Bytes;

#[cfg(feature = "moq")]
#[derive(Debug, Default)]
pub struct BinaryData {
    data: Bytes,
}
