mod error;

pub use error::{DecodeError, RecvError};
use varint::{VarInt, core::ReferenceReader};

use crate::transport::PACKET_SIZE;

use {
    error::{ctx, dec_ctx},
    snafu::ResultExt,
};

pub enum RecvStream {
    Quic(quic::RecvStream),
    WebTransport(webtransport::RecvStream),
}

impl RecvStream {
    /// Read data contiguously from the stream.
    ///
    /// Yields the number of bytes read into buf on success, or None if the stream was finished.
    ///
    /// This operation is cancel-safe.
    #[tracing::instrument(skip(self, buf), err)]
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<Option<usize>, RecvError> {
        match self {
            Self::Quic(rx) => rx.read(buf).await.context(ctx::QuicSnafu),
            Self::WebTransport(rx) => rx.read(buf).await.context(ctx::WebTransportSnafu),
        }
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn recv<M>(&mut self) -> Result<M, DecodeError>
    where
        M: VarInt,
    {
        let mut buf = vec![0; PACKET_SIZE];
        let len = self.read(&mut buf).await.context(dec_ctx::RecvSnafu)?.expect("# TODO what exactly means \"stream is finished\" closed or done sending current packet?");
        let buf = &buf[..len];

        let mut reader = ReferenceReader::new(buf);

        let (msg, _bits) =
            M::decode(&mut reader, Some(len * 8)).map_err(|err| DecodeError::VarInt {
                cause: err.to_string(),
            })?;

        Ok(msg)
    }
}
