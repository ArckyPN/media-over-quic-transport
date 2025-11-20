use snafu::Snafu;

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum RecvError {
    #[snafu(display("failed to read from stream"))]
    Quic { source: quic::ReadError },

    #[snafu(display("failed to read from stream"))]
    WebTransport {
        source: webtransport::error::StreamReadError,
    },
}

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(dec_ctx))]
pub enum DecodeError {
    #[snafu(display("failed to decode data: {cause}"))]
    VarInt { cause: String },

    #[snafu(display("failed to receive data"))]
    Recv { source: RecvError },
}
