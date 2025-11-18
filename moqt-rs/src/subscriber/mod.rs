mod builder;
mod error;

pub use error::SubscriberError;

use {crate::transport::Connection, error::ctx};

/// TODO docs
pub struct Subscriber {
    transport: Connection,
}
