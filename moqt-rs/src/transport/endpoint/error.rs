use snafu::Snafu;

use crate::macro_helper::impl_from_msg_error;

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum EndpointError {
    #[snafu(display("io: {msg}"))]
    IoError { msg: String },

    /// QUIC [ConnectionError](quic::ConnectionError)
    #[snafu(display("QUIC failed to connect"))]
    QuicConnection { source: quic::ConnectionError },

    /// WebTransport [ConnectionError](webtransport::error::ConnectionError)
    #[snafu(display("WebTransport failed to connect"))]
    WebTransportConnection {
        source: webtransport::error::ConnectionError,
    },
}

impl_from_msg_error! {
    EndpointError = [
        IoError => std::io::Error,
    ]
}
