mod builder;
mod config;
mod error;

pub use {config::ClientConfig, error::ClientError};

use {
    crate::{
        ControlStream,
        transport::Connection,
        types::{
            Name, Namespace, RequestId,
            config::SubscribeConfig,
            message::{ControlMessage, Subscribe},
        },
    },
    error::ctx,
    snafu::ResultExt,
    tokio::sync::RwLock,
    varint::x,
};

/// TODO docs
pub struct Client {
    transport: Connection,
    control_stream: ControlStream,
    request_id: RwLock<RequestId>,
}

impl Client {
    // TODO publish(namespace, name)
    // TODO subscribe(namespace, name)
    // TODO etc.
    pub async fn subscribe<S, N>(
        &self,
        namespace: S,
        name: N,
        config: Option<SubscribeConfig>,
    ) -> Result<(), ClientError>
    where
        S: Into<Namespace>,
        N: Into<Name>,
    {
        let msg = Subscribe::from_config()
            .id(self.next_id().await?)
            .namespace(namespace)
            .name(name)
            .config(config.unwrap_or_default())
            .build();

        self.control_stream
            .send(ControlMessage::Subscribe(msg))
            .await
            .context(ctx::ControlStreamSnafu)?;

        // TODO return a type to receive Objects from the subscription
        Ok(())
    }

    async fn next_id(&self) -> Result<x!(i), ClientError> {
        self.request_id
            .write()
            .await
            .get()
            .context(ctx::RequestLimitReachedSnafu)
    }
}
