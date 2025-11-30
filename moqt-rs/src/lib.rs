mod client;
mod control_stream;
mod macro_helper;
mod server;
mod session;
pub mod transport;
pub mod types;

pub use {
    client::{Client, ClientConfig},
    control_stream::ControlStream,
    server::{Protocol, Server, ServerConfig},
    session::Session,
};

/// the draft version this crate implements
pub const DRAFT_VERSION: u32 = 0xFF00000E;
pub const SUPPORTED_VERSION: &[u32] = &[DRAFT_VERSION];

pub mod error {
    pub use super::{
        client::ClientError, control_stream::ControlStreamError, server::ServerError,
        session::SessionError,
    };
}

#[cfg(test)]
mod test_helper;
