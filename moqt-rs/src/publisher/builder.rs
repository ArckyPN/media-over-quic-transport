use {
    super::{Connection, Publisher, PublisherConfig, PublisherError, ctx},
    crate::Protocol,
    bon::bon,
    snafu::ResultExt,
    webtransport::endpoint::IntoConnectOptions,
};

#[bon]
impl Publisher {
    /// Create a [Publisher] using a [PublisherConfig]
    pub async fn new(config: PublisherConfig) -> Result<Self, PublisherError> {
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
    ) -> Result<Self, PublisherError>
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

        // TODO perform handshake and set control stream on Self

        Ok(Self { transport })
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
    ) -> Result<Self, PublisherError>
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
