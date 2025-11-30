use {
    super::{Endpoint, Server, ServerConfig, ServerError, ctx},
    crate::Protocol,
    bon::bon,
    core::net::SocketAddr,
    snafu::ResultExt,
    std::{path::Path, sync::Arc},
};

#[bon]
impl Server {
    /// Create a [Server] using a [ServerConfig]
    #[tracing::instrument]
    pub async fn new(config: ServerConfig) -> Result<Self, ServerError> {
        match &config.protocol {
            Protocol::Quic => {
                Self::quic_builder()
                    .bind(config.bind)
                    .cert(&config.cert)
                    .key(&config.key)
                    .build()
                    .await
            }
            Protocol::WebTransport => {
                Self::webtransport_builder()
                    .bind(config.bind)
                    .cert(&config.cert)
                    .key(&config.key)
                    .build()
                    .await
            }
        }
    }

    /// Create a WebTransport [Server] using a Builder pattern
    #[builder(start_fn = webtransport_builder, finish_fn = build)]
    pub async fn new_webtransport<C, K>(
        #[builder(into, setters(doc {
            /// ## Bind Address
            /// 
            /// The Socket Address to listen on to
            /// receive WebTransport connections.
        }))]
        bind: SocketAddr,
        #[builder(setters(doc {
            /// ## TLS Certificate File
            /// 
            /// The Path to the cert file.
        }))]
        cert: C,
        #[builder(setters(doc {
            /// ## TLS Private Key File
            /// 
            /// The Path to the key file.
        }))]
        key: K,
    ) -> Result<Self, ServerError>
    where
        C: AsRef<Path>,
        K: AsRef<Path>,
    {
        Ok(Self {
            transport: Endpoint::webtransport_builder()
                .config(
                    webtransport::ServerConfig::builder()
                        .with_bind_address(bind)
                        .with_identity(webtransport::Identity::load_pemfiles(cert, key).await?)
                        .build(),
                )
                .build()
                .await
                .context(ctx::EndpointSnafu)?,
            sessions: Default::default(),
        })
    }

    /// Create a QUIC [Server] using a Builder pattern
    #[builder(start_fn = quic_builder, finish_fn = build)]
    pub async fn new_quic<C, K>(
        #[builder(into, setters(doc {
            /// ## Bind Address
            /// 
            /// The Socket Address to listen on to
            /// receive WebTransport connections.
        }))]
        bind: SocketAddr,
        #[builder(setters(doc {
            /// ## TLS Certificate File
            /// 
            /// The Path to the cert file.
        }))]
        cert: C,
        #[builder(setters(doc {
            /// ## TLS Private Key File
            /// 
            /// The Path to the key file.
        }))]
        key: K,
    ) -> Result<Self, ServerError>
    where
        C: AsRef<Path>,
        K: AsRef<Path>,
    {
        let tls_config = webtransport::tls::server::build_default_tls_config(
            webtransport::Identity::load_pemfiles(cert, key).await?,
        );

        let crypto = Arc::new(
            quic::crypto::rustls::QuicServerConfig::try_from(tls_config)
                .expect("CipherSuite::TLS13_AES_128_GCM_SHA256 missing"),
        );
        let mut config = quic::ServerConfig::with_crypto(crypto);
        config.transport_config(Arc::new(quic::TransportConfig::default()));
        config.migration(true);

        Ok(Self {
            transport: Endpoint::quic_builder()
                .bind(bind)
                .config(config)
                .build()
                .await
                .context(ctx::EndpointSnafu)?,
            sessions: Default::default(),
        })
    }
}
