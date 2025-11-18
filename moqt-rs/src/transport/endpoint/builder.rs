use {
    super::{Endpoint, EndpointError},
    bon::bon,
    core::net::SocketAddr,
};

#[bon]
impl Endpoint {
    /// TODO docs
    #[builder(start_fn = webtransport_builder, finish_fn = build)]
    pub async fn new_webtransport(
        #[builder(setters(doc {
            /// ## Server Config
            /// 
            /// Configuration of the WebTransport Server.
        }))]
        config: webtransport::ServerConfig,
    ) -> Result<Self, EndpointError> {
        Ok(Self::WebTransport(webtransport::Endpoint::server(config)?))
    }

    /// TODO docs
    #[builder(start_fn = quic_builder, finish_fn = build)]
    pub async fn new_quic(
        #[builder(setters(doc {
            /// ## Server Config
            /// 
            /// Configuration of the QUIC Server.
        }))]
        config: quic::ServerConfig,
        #[builder(setters(doc {
            /// ## Listen Address
            /// 
            /// Bind the Server to this Address to listen on.
        }))]
        bind: SocketAddr,
    ) -> Result<Self, EndpointError> {
        Ok(Self::Quic(quic::Endpoint::server(config, bind)?))
    }
}
