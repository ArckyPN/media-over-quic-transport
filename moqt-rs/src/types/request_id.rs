use {snafu::Snafu, varint::x};

#[derive(Debug)]
pub struct RequestId {
    id: u64,
}

impl RequestId {
    pub fn new_client() -> Self {
        Self { id: 0 }
    }

    pub fn new_server() -> Self {
        Self { id: 1 }
    }

    pub fn get(&mut self) -> Result<x!(i), RequestIdError> {
        let id = self.id;

        snafu::ensure!(id <= <x!(i)>::MAX, RequestIdSnafu);
        self.id += 2;

        // uncheck is possible here because we've ensured it
        // it not larger than MAX above
        Ok(<x!(i)>::new_unchecked(id))
    }

    pub fn is_client(&self) -> bool {
        self.id.is_multiple_of(2)
    }
}

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(display("request limit reached"))]
pub struct RequestIdError;
