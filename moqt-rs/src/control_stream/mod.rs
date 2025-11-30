mod error;

use std::sync::Arc;

pub use error::ControlStreamError;
use snafu::ResultExt;
use tokio::sync::Mutex;
use tracing::{debug, error, trace};
use varint::VarIntNumber;

use crate::{
    DRAFT_VERSION, SUPPORTED_VERSION,
    types::{
        error_code::Termination,
        message::{ClientSetup, ControlMessage, ServerSetup},
    },
};

use {
    crate::transport::{Connection, RecvStream, SendStream},
    error::ctx,
};

pub struct ControlStream {
    tx: Arc<Mutex<SendStream>>,
    rx: Arc<Mutex<RecvStream>>,
}

impl ControlStream {
    /// Opens the ControlStream (the one and only bidirectional Stream) and performs
    /// the MOQT handshake with it.
    #[tracing::instrument(skip(conn), err)]
    pub async fn open(conn: &Connection) -> Result<Self, ControlStreamError> {
        let (tx, rx) = conn.open_bi().await.context(ctx::ConnectionSnafu)?;
        debug!("opened ControlStream");
        trace!("initiating MOQT handshake");

        let this = Self {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
        };

        this.send(ControlMessage::ClientSetup(
            // TODO any parameters? probably for the QUIC specific ones
            ClientSetup::builder().version(DRAFT_VERSION).build(),
        ))
        .await?;
        debug!("ClientSetup sent");

        let msg = match this.recv().await? {
            ControlMessage::ServerSetup(msg) => msg,
            x => {
                error!(
                    key = x.key(),
                    "invalid ControlMessage, expected ServerSetup"
                );
                conn.close(Termination::ProtocolViolation);
                return Err(ControlStreamError::ProtocolViolation);
            }
        };
        debug!(?msg, "ServerSetup received");

        let selected_version = msg.selected_version::<u64>();
        snafu::ensure!(
            DRAFT_VERSION as u64 == selected_version,
            ctx::MismatchedVersionSnafu {
                client: DRAFT_VERSION,
                server: selected_version,
            }
        );

        debug!(version = selected_version, "established MOQT connection");

        Ok(this)
    }

    /// Accepts the ControlStream (the one and only bidirectional Stream) and performs
    /// the MOQT handshake with it.
    #[tracing::instrument(skip(conn), err)]
    pub async fn accept(conn: &Connection) -> Result<Self, ControlStreamError> {
        let (tx, rx) = conn.accept_bi().await.context(ctx::ConnectionSnafu)?;
        debug!("accepted ControlStream");

        let this = Self {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
        };

        let msg = match this.recv().await? {
            ControlMessage::ClientSetup(msg) => msg,
            x => {
                error!(
                    key = x.key(),
                    "invalid ControlMessage, expected ClientSetup"
                );
                conn.close(Termination::ProtocolViolation);
                return Err(ControlStreamError::ProtocolViolation);
            }
        };
        debug!(?msg, "ClientSetup received");

        let supported_version = msg.supported_version(SUPPORTED_VERSION).ok_or(
            ControlStreamError::NoSupportedVersion {
                client: msg
                    .supported_versions
                    .iter()
                    .map(VarIntNumber::number)
                    .collect(),
                server: SUPPORTED_VERSION.iter().map(|v| *v as u64).collect(),
            },
        )?;
        debug!(version = %supported_version, "version negotiated");

        this.send(ControlMessage::ServerSetup(
            // TODO any parameters?
            ServerSetup::builder().version(supported_version).build(),
        ))
        .await?;
        debug!("ClientServerSetup sent");

        Ok(this)
    }

    /// Sends a [ControlMessage].
    #[tracing::instrument(skip(self), err)]
    pub async fn send(&self, msg: ControlMessage) -> Result<(), ControlStreamError> {
        let mut lock = self.tx.lock().await;

        lock.send(msg).await.context(ctx::SendSnafu)
    }

    /// Receives a [ControlMessage].
    #[tracing::instrument(skip(self), err)]
    pub async fn recv(&self) -> Result<ControlMessage, ControlStreamError> {
        let mut lock = self.rx.lock().await;

        lock.recv().await.context(ctx::RecvSnafu)
    }
}
