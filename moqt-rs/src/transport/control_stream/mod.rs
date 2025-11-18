mod error;

pub use error::ControlStreamError;

use {
    crate::transport::{Connection, RecvStream, SendStream},
    error::ctx,
};

pub struct ControlStream {
    tx: SendStream,
    rx: RecvStream,
}

impl ControlStream {
    pub fn open(conn: &Connection) -> Result<Self, ControlStreamError> {
        todo!()
    }

    pub fn accept(conn: &Connection) -> Result<Self, ControlStreamError> {
        todo!()
    }
}
