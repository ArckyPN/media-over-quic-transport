use {
    crate::Protocol,
    bon::Builder,
    clap::Parser,
    serde::{Deserialize, Serialize},
    url::Url,
};

#[derive(Debug, Parser, Deserialize, Serialize, Builder, Clone)]
pub struct ClientConfig {
    /// The relay URL to connect to
    #[arg(short = 'r', long = "relay")]
    #[builder(into, setters(doc {
        /// ## Relay URL
        /// 
        /// The relay URL to connect to
    }))]
    pub relay: Url,

    /// Whether to use QUIC or WebTransport
    #[arg(short = 'p', long = "proto")]
    #[builder(into, setters(doc {
        /// ## Transport Protocol
        /// 
        /// Whether to use QUIC or WebTransport
    }))]
    pub protocol: Protocol,
}
