use {
    crate::{
        error::ControlStreamError, transport::error::ConnectionError, types::error::RequestIdError,
    },
    snafu::Snafu,
};

/// TODO docs
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum ClientError {
    /// TODO docs
    /// TODO snafu display
    #[snafu(display("client failed to connect"))]
    Connection { source: ConnectionError },

    #[snafu(display("client's ControlStream ran into an error"))]
    ControlStream { source: ControlStreamError },

    #[snafu(display("maximum request ID reached"))]
    RequestLimitReached { source: RequestIdError },
}
