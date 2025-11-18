use {crate::transport::ConnectionError, snafu::Snafu};

/// [Subscriber](crate::transport::Subscriber) Error
#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
#[snafu(visibility(pub), module(ctx))]
pub enum SubscriberError {
    /// QUIC or WebTransport Error
    #[snafu(display("protocol error"))]
    Connection { source: ConnectionError },
}
