pub enum RecvStream {
    Quic(quic::RecvStream),
    WebTransport(webtransport::RecvStream),
}
