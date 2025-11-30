use snafu::Snafu;
use varint::core::WriterError;

use crate::macro_helper::impl_from_msg_error;

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum SendError {
    #[snafu(display("failed to write data to stream"))]
    Quic {
        source: quic::WriteError,
    },

    #[snafu(display("failed to write data to stream"))]
    WebTransport {
        source: webtransport::error::StreamWriteError,
    },

    VarInt {
        cause: String,
    },

    Writer {
        source: WriterError,
    },
}

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(enc_ctx))]
pub enum EncodeError {
    #[snafu(display("failed to encode data: {msg}"))]
    VarInt { msg: String },

    #[snafu(display("failed to send data"))]
    Send { source: SendError },
}

impl_from_msg_error! {
    EncodeError = [
        VarInt => WriterError,
    ]
}
