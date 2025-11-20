mod error;

pub use error::SendError;

use {
    error::ctx,
    snafu::ResultExt,
    varint::{VarInt, Writer, core::ReferenceWriter},
};

pub enum SendStream {
    Quic(quic::SendStream),
    WebTransport(webtransport::SendStream),
}

impl SendStream {
    /// Writes a byte `buf`fer to the stream using `write_all`
    /// ([QUIC](quic::SendStream::write_all) / [WebTransport](webtransport::SendStream::write_all))
    /// functions, ensuring everything is sent.
    #[tracing::instrument(skip(self, buf), err)]
    pub async fn write(&mut self, buf: &[u8]) -> Result<(), SendError> {
        match self {
            Self::Quic(tx) => tx.write_all(buf).await.context(ctx::QuicSnafu),
            Self::WebTransport(tx) => tx.write_all(buf).await.context(ctx::WebTransportSnafu),
        }
    }

    /// Encodes any type implementing [VarInt] and writes it to
    /// the stream.
    #[tracing::instrument(skip(self, v), err)] // TODO make a trait Message: VarInt, which adds a name() -> &'static string for logging
    pub async fn send<V>(&mut self, v: V) -> Result<(), SendError>
    // TODO make a separate EncodeError
    where
        V: VarInt,
    {
        let mut writer = ReferenceWriter::new();

        let _bits = v
            .encode(
                &mut writer,
                Some(v.len_bits().map_err(|err| SendError::VarInt {
                    cause: err.to_string(),
                })?),
            )
            .map_err(|err| SendError::VarInt {
                cause: err.to_string(),
            })?;

        self.write(&writer.finish().context(ctx::WriterSnafu)?)
            .await
    }
}
