use snafu::Snafu;

use crate::transport::error::{ConnectionError, DecodeError, EncodeError, RecvError, SendError};

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum ControlStreamError {
    #[snafu(display("connection failed"))]
    Connection { source: ConnectionError },

    #[snafu(display("failed to send Message on the stream"))]
    Send { source: EncodeError },

    #[snafu(display("failed to receive Message on the stream"))]
    Recv { source: DecodeError },

    #[snafu(display("failed negotiate version: client: {client:?}, server: {server:?}"))]
    NoSupportedVersion { client: Vec<u64>, server: Vec<u64> },

    #[snafu(display("server responded with a mismatched version, {client} != {server}"))]
    MismatchedVersion { client: u32, server: u64 },

    #[snafu(display("MOQT Protocol violated"))]
    ProtocolViolation,
}
