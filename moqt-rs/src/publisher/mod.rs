mod builder;
mod config;
mod error;

pub use {config::PublisherConfig, error::PublisherError};

use {crate::transport::Connection, error::ctx};

/// TODO docs
pub struct Publisher {
    transport: Connection,
}

impl Publisher {
    // TODO publish(namespace, name)
}
