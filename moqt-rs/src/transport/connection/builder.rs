use {
    super::{Connection, ConnectionError, ctx},
    bon::bon,
    core::net::SocketAddr,
    snafu::ResultExt,
    std::net::{SocketAddrV4, SocketAddrV6},
    url::Url,
    webtransport::{
        config::{DnsResolver, TokioDnsResolver},
        endpoint::IntoConnectOptions,
    },
};

#[bon]
impl Connection {
    /// TODO docs
    #[builder(start_fn = webtransport_builder, finish_fn = build)]
    pub async fn new_webtransport<O>(
        #[builder(setters(doc {
            /// ## Client Config
            /// 
            /// Configuration of the WebTransport Client.
        }))]
        config: webtransport::ClientConfig,
        #[builder(setters(doc {
            /// ## Connection URL
            /// 
            /// The Endpoint will connect this WebTransport server.
        }))]
        connect: O,
    ) -> Result<Self, ConnectionError>
    where
        O: IntoConnectOptions,
    {
        Ok(Self::WebTransport(
            webtransport::Endpoint::client(config)?
                .connect(connect)
                .await?,
        ))
    }

    /// TODO docs
    #[builder(start_fn = quic_builder, finish_fn = build)]
    pub async fn new_quic<O>(
        #[builder(setters(doc {
            /// ## Client Config
            /// 
            /// Configuration of the QUIC Client.
        }))]
        config: quic::ClientConfig,
        #[builder(setters(doc {
            /// ## Connection URL
            /// 
            /// The Endpoint will connect this QUIC server.
        }))]
        connect: O,
    ) -> Result<Self, ConnectionError>
    where
        O: IntoConnectOptions,
    {
        // extract QUIC connection information from connect URL
        // adapted from source of: https://docs.rs/wtransport/latest/wtransport/struct.Endpoint.html#method.connect
        let options = connect.into_options();

        let url_str = options.url().to_owned();

        let url = Url::parse(options.url()).context(ctx::InvalidUrlSnafu {
            url: url_str.clone(),
        })?;
        snafu::ensure!(
            url.scheme() == "https",
            ctx::OtherSnafu {
                msg: "url must have 'https' scheme".to_owned()
            }
        );

        let host = url.host().expect("https scheme must have an host");
        let port = url.port().unwrap_or(443);

        let (socket_addr, host) = match host {
            url::Host::Domain(domain) => {
                let socket_addr = TokioDnsResolver
                    .resolve(&format!("{domain}:{port}"))
                    .await
                    .map_err(|err| ConnectionError::Other {
                        msg: format!("DNS lookup error: {err}"),
                    })?
                    .ok_or(ConnectionError::Other {
                        msg: "DNS not found".to_owned(),
                    })?;
                (socket_addr, domain.to_string())
            }
            url::Host::Ipv4(addr) => {
                let socket_addr = SocketAddr::V4(SocketAddrV4::new(addr, port));
                (socket_addr, addr.to_string())
            }
            url::Host::Ipv6(addr) => {
                let socket_addr = SocketAddr::V6(SocketAddrV6::new(addr, port, 0, 0));
                (socket_addr, addr.to_string())
            }
        };

        let mut endpoint =
            quic::Endpoint::client("[::]:0".parse().expect("valid IPv6 SocketAddr"))?;
        endpoint.set_default_client_config(config);

        Ok(Self::Quic(
            endpoint
                .connect(socket_addr, &host)
                .context(ctx::QuicConnectSnafu)?
                .await
                .context(ctx::QuicConnectionSnafu)?,
        ))
    }
}
