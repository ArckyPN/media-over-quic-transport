mod builder;
mod error;

pub use error::ConnectionError;

use {
    super::{RecvStream, SendStream},
    crate::types::error_code::Termination,
    core::net::SocketAddr,
    error::ctx,
    snafu::ResultExt,
    tracing::trace,
};

/// Wrapper Type around [QUIC](quic::Connection) and
/// [WebTransport](webtransport::Connection) Connection
/// types to support both protocols.
pub enum Connection {
    Quic(quic::Connection),
    WebTransport(webtransport::Connection),
}

impl Connection {
    /// Whether or not QUIC is in use.
    ///
    /// If not then WebTransport is used.
    pub fn is_quic(&self) -> bool {
        matches!(self, Self::Quic(_))
    }

    /// Open an outgoing unidirectional Stream
    #[tracing::instrument(skip(self), err)]
    pub async fn open_uni(&self) -> Result<SendStream, ConnectionError> {
        Ok(match self {
            Self::Quic(conn) => {
                let tx = conn.open_uni().await.context(ctx::QuicConnectionSnafu)?;

                trace!("opened unidirectional stream");

                SendStream::Quic(tx)
            }
            Self::WebTransport(conn) => {
                let tx = conn
                    .open_uni()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?
                    .await
                    .context(ctx::WebTransportOpeningStreamSnafu)?;

                trace!("opened unidirectional stream");

                SendStream::WebTransport(tx)
            }
        })
    }

    /// Accept an incoming unidirectional Stream
    #[tracing::instrument(skip(self), err)]
    pub async fn accept_uni(&self) -> Result<RecvStream, ConnectionError> {
        Ok(match self {
            Self::Quic(conn) => {
                let rx = conn.accept_uni().await.context(ctx::QuicConnectionSnafu)?;

                trace!("accepted unidirectional stream");

                RecvStream::Quic(rx)
            }
            Self::WebTransport(conn) => {
                let rx = conn
                    .accept_uni()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?;

                trace!("accepted unidirectional stream");

                RecvStream::WebTransport(rx)
            }
        })
    }

    /// Open an outgoing bidirectional Stream
    #[tracing::instrument(skip(self), err)]
    pub async fn open_bi(&self) -> Result<(SendStream, RecvStream), ConnectionError> {
        Ok(match self {
            Self::Quic(conn) => {
                let (tx, rx) = conn.open_bi().await.context(ctx::QuicConnectionSnafu)?;

                trace!("opened bidirectional stream");

                (SendStream::Quic(tx), RecvStream::Quic(rx))
            }
            Self::WebTransport(conn) => {
                let (tx, rx) = conn
                    .open_bi()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?
                    .await
                    .context(ctx::WebTransportOpeningStreamSnafu)?;

                trace!("opened bidirectional stream");

                (SendStream::WebTransport(tx), RecvStream::WebTransport(rx))
            }
        })
    }

    /// Accept an incoming bidirectional Stream
    #[tracing::instrument(skip(self), err)]
    pub async fn accept_bi(&self) -> Result<(SendStream, RecvStream), ConnectionError> {
        Ok(match self {
            Self::Quic(conn) => {
                let (tx, rx) = conn.accept_bi().await.context(ctx::QuicConnectionSnafu)?;

                trace!("accepted bidirectional stream");

                (SendStream::Quic(tx), RecvStream::Quic(rx))
            }
            Self::WebTransport(conn) => {
                let (tx, rx) = conn
                    .accept_bi()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?;

                trace!("accepted bidirectional stream");

                (SendStream::WebTransport(tx), RecvStream::WebTransport(rx))
            }
        })
    }

    pub fn remote_addr(&self) -> SocketAddr {
        match self {
            Self::Quic(conn) => conn.remote_address(),
            Self::WebTransport(conn) => conn.remote_address(),
        }
    }

    pub fn close(&self, code: Termination) {
        match self {
            Self::Quic(conn) => conn.close(code.key().into(), code.to_string().as_bytes()),
            Self::WebTransport(conn) => conn.close(code.key().into(), code.to_string().as_bytes()),
        }
    }
}
