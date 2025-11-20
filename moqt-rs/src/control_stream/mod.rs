mod error;

pub use error::ControlStreamError;
use snafu::ResultExt;
use tracing::{debug, trace};
use varint::VarIntNumber;

use crate::{
    DRAFT_VERSION, SUPPORTED_VERSION,
    types::message::{ClientSetup, ServerSetup},
};

use {
    crate::transport::{Connection, RecvStream, SendStream},
    error::ctx,
};

pub struct ControlStream {
    tx: SendStream,
    rx: RecvStream,
}

impl ControlStream {
    /// Opens the ControlStream (the one and only bidirectional Stream) and performs
    /// the MOQT handshake with it.
    #[tracing::instrument(skip(conn), err)]
    pub async fn open(conn: &Connection) -> Result<Self, ControlStreamError> {
        let (mut tx, mut rx) = conn.open_bi().await.context(ctx::ConnectionSnafu)?;
        debug!("opened ControlStream");
        trace!("initiating MOQT handshake");

        // TODO any parameters?
        tx.send(ClientSetup::builder().version(DRAFT_VERSION).build())
            .await
            .context(ctx::SendSnafu)?;
        debug!("ClientSetup sent");

        let msg: ServerSetup = rx.recv().await.context(ctx::DecodeSnafu)?;
        debug!(?msg, "ServerSetup received");

        snafu::ensure!(
            DRAFT_VERSION as u64 == msg.selected_version::<u64>(),
            ctx::MismatchedVersionSnafu {
                client: DRAFT_VERSION,
                server: msg.selected_version::<u64>()
            }
        );

        Ok(Self { tx, rx })
    }

    /// Accepts the ControlStream (the one and only bidirectional Stream) and performs
    /// the MOQT handshake with it.
    pub async fn accept(conn: &Connection) -> Result<Self, ControlStreamError> {
        let (mut tx, mut rx) = conn.accept_bi().await.context(ctx::ConnectionSnafu)?;
        debug!("accepted ControlStream");

        let msg: ClientSetup = rx.recv().await.context(ctx::DecodeSnafu)?;
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

        // TODO any parameters?
        tx.send(ServerSetup::builder().version(supported_version).build())
            .await
            .context(ctx::SendSnafu)?;
        debug!("ClientServerSetupetup sent");

        Ok(Self { tx, rx })
    }
}
