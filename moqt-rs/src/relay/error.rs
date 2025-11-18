use {
    crate::{macro_helper::impl_from_msg_error, transport::EndpointError},
    snafu::Snafu,
};

/// [Relay](crate::transport::Relay) Error
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum RelayError {
    /// unable to load TLS files
    #[snafu(display("failed to load TLS files: {msg}"))]
    TlsLoad { msg: String },

    /// QUIC or WebTransport Error
    #[snafu(display("protocol error"))]
    Endpoint { source: EndpointError },
}

impl_from_msg_error! {
    RelayError = [
        TlsLoad => webtransport::tls::error::PemLoadError
    ]
}
