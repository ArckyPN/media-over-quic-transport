use {
    crate::{
        error::ControlStreamError,
        macro_helper::impl_from_msg_error,
        transport::error::{ConnectionError, EndpointError},
    },
    snafu::Snafu,
};

/// [Relay](crate::transport::Relay) Error
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum ServerError {
    /// unable to load TLS files
    #[snafu(display("failed to load TLS files: {msg}"))]
    TlsLoad { msg: String },

    /// QUIC or WebTransport Error
    #[snafu(display("protocol error"))]
    Endpoint { source: EndpointError },

    /// ControlStream Error
    #[snafu(display("failed to establish the ControlStream"))]
    ControlStream { source: ControlStreamError },
}

impl_from_msg_error! {
    ServerError = [
        TlsLoad => webtransport::tls::error::PemLoadError
    ]
}
