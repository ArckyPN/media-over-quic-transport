use snafu::Snafu;

use crate::transport::error::{ConnectionError, DecodeError, RecvError, SendError};

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum ControlStreamError {
    #[snafu(display("connection failed"))]
    Connection { source: ConnectionError },

    #[snafu(display("failed to write data to the stream"))]
    Send { source: SendError },

    #[snafu(display("failed to read data from the stream"))]
    Recv { source: RecvError },

    // TODO add message name?
    #[snafu(display("failed to receive Message on the stream"))]
    Decode { source: DecodeError },

    #[snafu(display("failed negotiate version: client: {client:?}, server: {server:?}"))]
    NoSupportedVersion { client: Vec<u64>, server: Vec<u64> },

    #[snafu(display("server responded with a mismatched version, {client} != {server}"))]
    MismatchedVersion { client: u32, server: u64 },
}
