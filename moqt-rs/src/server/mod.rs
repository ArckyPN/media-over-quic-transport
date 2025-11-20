mod builder;
mod config;
mod error;

pub use {
    config::{Protocol, ServerConfig},
    error::ServerError,
};

use {
    crate::transport::Endpoint,
    error::ctx,
    tracing::{error, info},
};

/// A Server is the MOQT **Relay**.
///
/// It is named Server to keep it consistent
/// with typical naming schemes.
/// TODO docs
#[derive(Debug)]
pub struct Server {
    transport: Endpoint,
}

impl Server {
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
            let conn = match self.transport.accept().await {
                Ok(con) => con,
                Err(err) => {
                    error!(%err, "failed to accept incoming session");
                    continue;
                }
            };

            // TODO tokio spawn move conn to session and and do the handling there
            let control_stream = crate::ControlStream::accept(&conn)
                .await
                .expect("ControlStream failed");

            // TODO handle session
            // TODO session type, which handles the handshake on stuff
        }
    }
}
