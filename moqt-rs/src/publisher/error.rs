use {crate::transport::ConnectionError, snafu::Snafu};

/// TODO docs
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum PublisherError {
    /// TODO docs
    /// TODO snafu display
    Connection { source: ConnectionError },
}
