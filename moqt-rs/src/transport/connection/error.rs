use snafu::Snafu;

use crate::macro_helper::impl_from_msg_error;

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum ConnectionError {
    #[snafu(display("io: {msg}"))]
    IoError { msg: String },

    #[snafu(display("webtransport failed to connect: {msg}"))]
    WebTransportConnecting { msg: String },

    #[snafu(display("WebTransport connection failed"))]
    WebTransportConnection {
        source: webtransport::error::ConnectionError,
    },

    #[snafu(display("WebTransport failed to open a Stream"))]
    WebTransportOpeningStream {
        source: webtransport::error::StreamOpeningError,
    },

    #[snafu(display("QUIC failed to connect"))]
    QuicConnect { source: quic::ConnectError },

    #[snafu(display("QUIC connection failed"))]
    QuicConnection { source: quic::ConnectionError },

    #[snafu(display("{url} is an invalid URL"))]
    InvalidUrl {
        url: String,
        source: url::ParseError,
    },

    #[snafu(display("{msg}"))]
    Other { msg: String },
}

impl_from_msg_error! {
    ConnectionError = [
        IoError => std::io::Error,
        WebTransportConnecting => webtransport::error::ConnectingError
    ]
}
