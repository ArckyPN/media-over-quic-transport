//! Wrapper types around QUIC (quinn) and WebTransport (wtransport)
//! types to have a unified API to use either of them.

mod connection;
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

pub mod error {
    pub use super::{
        connection::ConnectionError,
        endpoint::EndpointError,
        recv_stream::{DecodeError, RecvError},
        send_stream::{EncodeError, SendError},
    };
}

pub(crate) use {
    connection::Connection, endpoint::Endpoint, recv_stream::RecvStream, send_stream::SendStream,
};

const WEBTRANSPORT: &str = "WebTransport";
const QUIC: &str = "QUIC";
const PACKET_SIZE: usize = (1 << 16) - 1;
