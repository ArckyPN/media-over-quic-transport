use {
    bon::Builder,
    clap::Parser,
    core::net::SocketAddr,
    serde::{Deserialize, Serialize},
    std::path::PathBuf,
    strum_lite::strum,
};

/// ## Configuration of [Relay](crate::Relay)
///
/// Derives:
/// - [clap] CLI Parser
/// - [bon] Builder pattern
/// - [serde] Serialize and Deserialize
#[derive(Debug, Parser, Deserialize, Serialize, Builder, Clone)]
pub struct RelayConfig {
    /// The relay will listen on this address
    #[arg(short = 'b', long = "bind")]
    #[builder(into, setters(doc {
        /// ## Bind Address
        /// 
        /// The Socket Address to listen on to
        /// receive incoming connections
    }))]
    pub bind: SocketAddr,

    /// Path to the TLS certificate file
    #[arg(short = 'c', long = "cert")]
    #[builder(into, setters(doc {
        /// ## TLS certificate
        /// 
        /// Path to the TLS certificate file
    }))]
    pub cert: PathBuf,

    /// Path to the TLS private key file
    #[arg(short = 'k', long = "key")]
    #[builder(into, setters(doc {
        /// ## TLS private key
        /// 
        /// Path to the TLS private key file
    }))]
    pub key: PathBuf,

    /// Whether to use QUIC or WebTransport
    #[arg(short = 'p', long = "proto")]
    #[builder(into, setters(doc {
        /// ## Transport Protocol
        /// 
        /// Whether to use QUIC or WebTransport
    }))]
    pub protocol: Protocol,
}

strum! {
    /// Configures the Transport Protocol
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    pub enum Protocol {
        Quic = "quic" | "q",
        WebTransport = "webtransport" | "wt" | "w",
    }
}
