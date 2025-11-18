mod builder;
mod error;

pub use error::ConnectionError;
use snafu::ResultExt;

use {
    super::{ControlStream, RecvStream, SendStream},
    error::ctx,
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

    #[tracing::instrument(skip(self), err)]
    pub async fn handshake(&self) -> Result<ControlStream, ConnectionError> {
        match self {
            Self::Quic(conn) => {}
            Self::WebTransport(conn) => {
                let cs = conn
                    .open_bi()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?
                    .await
                    .context(ctx::WebTransportOpeningStreamSnafu)?;
            }
        }
        todo!()
    }

    /// Open an outgoing unidirectional Stream
    #[tracing::instrument(skip(self), err)]
    pub async fn open_uni(&self) -> Result<SendStream, ConnectionError> {
        Ok(match self {
            Self::Quic(conn) => {
                let tx = conn.open_uni().await.context(ctx::QuicConnectionSnafu)?;

                // TODO log

                SendStream::Quic(tx)
            }
            Self::WebTransport(conn) => {
                let tx = conn
                    .open_uni()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?
                    .await
                    .context(ctx::WebTransportOpeningStreamSnafu)?;

                // TODO log

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

                // TODO log

                RecvStream::Quic(rx)
            }
            Self::WebTransport(conn) => {
                let rx = conn
                    .accept_uni()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?;

                // TODO log

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

                // TODO log

                (SendStream::Quic(tx), RecvStream::Quic(rx))
            }
            Self::WebTransport(conn) => {
                let (tx, rx) = conn
                    .open_bi()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?
                    .await
                    .context(ctx::WebTransportOpeningStreamSnafu)?;

                // TODO log

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

                // TODO log

                (SendStream::Quic(tx), RecvStream::Quic(rx))
            }
            Self::WebTransport(conn) => {
                let (tx, rx) = conn
                    .accept_bi()
                    .await
                    .context(ctx::WebTransportConnectionSnafu)?;

                // TODO log

                (SendStream::WebTransport(tx), RecvStream::WebTransport(rx))
            }
        })
    }
}
