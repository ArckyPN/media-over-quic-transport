//! Wrapper types around QUIC (quinn) and WebTransport (wtransport)
//! types to have a unified API to use either of them.

mod connection;
mod control_stream;
mod endpoint;
mod recv_stream;
mod send_stream;

/// Re-exports of required wtransport types
pub mod webtransport {
    pub use webtransport::{ClientConfig, ServerConfig};
}
/// Re-exports of required quinn types
pub mod quic {
    pub use quic::{ClientConfig, ServerConfig};
}

pub(crate) use {
    connection::{Connection, ConnectionError},
    control_stream::ControlStream,
    endpoint::{Endpoint, EndpointError},
    recv_stream::RecvStream,
    send_stream::SendStream,
};

const WEBTRANSPORT: &str = "WebTransport";
const QUIC: &str = "QUIC";
