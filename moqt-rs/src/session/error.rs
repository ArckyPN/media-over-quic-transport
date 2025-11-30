use snafu::Snafu;
use tokio::sync::mpsc::error::{SendError, TryRecvError};

use {super::SessionMessage, crate::error::ControlStreamError};

/// TODO docs
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum SessionError {
    ControlStream { source: ControlStreamError },

    SessionHandle,
}

/// TODO docs
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(hs_ctx))]
pub enum SessionHandleError {
    #[snafu(display("session handle channel unexpectedly closed"))]
    ChannelClosed { source: TryRecvError },

    #[snafu(display("failed to send message on channel"))]
    ChannelSend { source: SendError<SessionMessage> },
}
