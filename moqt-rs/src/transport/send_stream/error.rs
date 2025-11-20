use snafu::Snafu;
use varint::core::WriterError;

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
