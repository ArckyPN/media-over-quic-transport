use {
    super::{Client, ClientConfig, ClientError, Connection, ctx},
    crate::{ControlStream, Protocol, types::RequestId},
    bon::bon,
    snafu::ResultExt,
    tokio::sync::RwLock,
    tracing::debug,
    webtransport::endpoint::IntoConnectOptions,
};

#[bon]
impl Client {
    /// Create a [Publisher] using a [PublisherConfig]
    #[tracing::instrument]
    pub async fn new(config: ClientConfig) -> Result<Self, ClientError> {
        match &config.protocol {
            Protocol::Quic => Self::quic_builder().connect(config.relay).build().await,
            Protocol::WebTransport => {
                Self::webtransport_builder()
                    .connect(config.relay)
                    .build()
                    .await
            }
        }
    }

    /// Create a WebTransport [Publisher] using a Builder pattern
    #[builder(start_fn = webtransport_builder, finish_fn = build)]
    pub async fn new_webtransport<O>(
        #[builder(setters(doc {
            /// ## Connection URL
            /// 
            /// The Endpoint will connect this WebTransport server.
        }))]
        connect: O,
    ) -> Result<Self, ClientError>
    where
        O: IntoConnectOptions,
    {
        // establish he connection to the relay
        let transport = Connection::webtransport_builder()
            .config(webtransport::ClientConfig::default())
            .connect(connect)
            .build()
            .await
            .context(ctx::ConnectionSnafu)?;
        debug!("connection established");

        let control_stream = ControlStream::open(&transport)
            .await
            .context(ctx::ControlStreamSnafu)?;

        Ok(Self {
            transport,
            control_stream,
            request_id: RwLock::new(RequestId::new_client()),
        })
    }

    /// Create a QUIC [Publisher] using a Builder pattern
    #[builder(start_fn = quic_builder, finish_fn = build)]
    pub async fn new_quic<O>(
        #[builder(setters(doc {
            /// ## Connection URL
            /// 
            /// The Endpoint will connect this QUIC server.
        }))]
        _connect: O,
    ) -> Result<Self, ClientError>
    where
        O: IntoConnectOptions,
    {
        todo!("# TODO quic support")
        // Ok(Self {
        //     transport: Connection::quic_builder().config(quic::ClientConfig::new(Arc::new(data)))
        //         .context(ctx::ConnectionSnafu)?,
        // })
    }
}
