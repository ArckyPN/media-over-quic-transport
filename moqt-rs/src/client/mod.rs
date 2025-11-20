mod builder;
mod config;
mod error;

pub use {config::ClientConfig, error::ClientError};

use {
    crate::{ControlStream, transport::Connection},
    error::ctx,
};

// TODO one Client instead of separate Subscribe/Publisher
/// TODO docs
pub struct Client {
    transport: Connection,
    control_stream: ControlStream,
}

impl Client {
    // TODO publish(namespace, name)
    // TODO subscribe(namespace, name)
    // TODO etc.
}
