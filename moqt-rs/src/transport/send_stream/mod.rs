pub enum SendStream {
    Quic(quic::SendStream),
    WebTransport(webtransport::SendStream),
}

impl SendStream {}
