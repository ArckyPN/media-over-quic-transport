// TODO turn this into an macro, which only take Publisher/Subscriber ident, all this is 100% identically between the two
use {
    super::{Connection, Subscriber, SubscriberError, ctx},
    bon::bon,
    snafu::ResultExt,
    webtransport::endpoint::IntoConnectOptions,
};

#[bon]
impl Subscriber {
    /// TODO docs
    #[builder(start_fn = webtransport_builder, finish_fn = build)]
    pub async fn new_webtransport<O>(
        #[builder(setters(doc {
            /// ## Connection URL
            /// 
            /// The Endpoint will connect this WebTransport server.
        }))]
        connect: O,
    ) -> Result<Self, SubscriberError>
    where
        O: IntoConnectOptions,
    {
        Ok(Self {
            transport: Connection::webtransport_builder()
                .config(webtransport::ClientConfig::default())
                .connect(connect)
                .build()
                .await
                .context(ctx::ConnectionSnafu)?,
        })
    }

    /// TODO docs
    #[builder(start_fn = quic_builder, finish_fn = build)]
    pub async fn new_quic<O>(
        #[builder(setters(doc {
            /// ## Connection URL
            /// 
            /// The Endpoint will connect this QUIC server.
        }))]
        _connect: O,
    ) -> Result<Self, SubscriberError>
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
