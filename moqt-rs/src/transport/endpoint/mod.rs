mod builder;
mod error;

pub use error::EndpointError;
use snafu::ResultExt;
use tracing::{info, trace};

use crate::transport::{QUIC, WEBTRANSPORT};

use {
    crate::transport::Connection, core::net::SocketAddr, error::ctx, std::fmt::Debug,
    webtransport::endpoint::endpoint_side::Server,
};

/// TODO docs
pub enum Endpoint {
    Quic(quic::Endpoint),
    WebTransport(webtransport::Endpoint<Server>),
}

impl Endpoint {
    /// Returns the name of the Protocol in use.
    pub fn proto(&self) -> &'static str {
        match self {
            Self::Quic(_) => QUIC,
            Self::WebTransport(_) => WEBTRANSPORT,
        }
    }

    /// Returns the local address of the Endpoint.
    pub fn local_address(&self) -> Result<SocketAddr, EndpointError> {
        Ok(match self {
            Self::Quic(ep) => ep.local_addr()?,
            Self::WebTransport(ep) => ep.local_addr()?,
        })
    }

    #[tracing::instrument(skip(self), fields(remote_addr), err)]
    pub async fn accept(&self) -> Result<Connection, EndpointError> {
        match self {
            Self::Quic(ep) => {
                let connection = loop {
                    if let Some(incoming) = ep.accept().await {
                        trace!("receiving a new session");
                        break incoming.await.context(ctx::QuicConnectionSnafu)?;
                    }
                };

                tracing::Span::current()
                    .record("remote_addr", &connection.remote_address().to_string());
                info!("new session accepted");

                Ok(Connection::Quic(connection))
            }
            Self::WebTransport(ep) => {
                let incoming_session = ep.accept().await;
                trace!("receiving a new session");

                let incoming_request = incoming_session
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?;
                trace!("receiving requests");

                tracing::Span::current().record(
                    "remote_addr",
                    &incoming_request.remote_address().to_string(),
                );

                let connection = incoming_request
                    .accept()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?;
                info!("new session accepted");

                Ok(Connection::WebTransport(connection))
            }
        }
    }
}

impl Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quic(ep) => f.debug_tuple("QUIC").field(ep).finish(),
            Self::WebTransport(_ep) => f
                .debug_tuple("WebTransport")
                .field(&"/* inaccessible */")
                .finish(),
        }
    }
}
