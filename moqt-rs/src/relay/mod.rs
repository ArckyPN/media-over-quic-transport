mod builder;
mod config;
mod error;

pub use {
    config::{Protocol, RelayConfig},
    error::RelayError,
};

use {
    crate::transport::Endpoint,
    error::ctx,
    tracing::{error, info},
};

/// TODO docs
#[derive(Debug)]
pub struct Relay {
    transport: Endpoint,
}

impl Relay {
    /// Launches the [Relay] making it run until
    /// termination or encountering an fatal error.
    #[tracing::instrument(skip(self))]
    pub async fn run(&self) {
        info!(
            addr = ?self.transport.local_address(),
            proto = self.transport.proto(),
            "Relay is running"
        );

        loop {
            let session = match self.transport.accept().await {
                Ok(con) => con,
                Err(err) => {
                    error!(%err, "failed to accept incoming session");
                    continue;
                }
            };

            // TODO handle session
        }
    }
}
