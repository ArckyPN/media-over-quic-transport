use {super::BinaryData, crate::VarIntBytes};

impl VarIntBytes for BinaryData {
    fn bytes(&self) -> bytes::Bytes {
        self.data.clone()
    }

    fn new_bytes(buf: &[u8], _n: Option<usize>) -> Result<Self, Self::Error> {
        Ok(Self::from(buf.to_vec()))
    }

    fn set_bytes(&mut self, buf: &[u8], _n: Option<usize>) -> Result<&mut Self, Self::Error> {
        self.data = buf.to_vec().into();
        Ok(self)
    }
}
