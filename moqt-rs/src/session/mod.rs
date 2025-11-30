mod error;
mod handle;
mod message;

pub use {
    error::{SessionError, SessionHandleError},
    handle::SessionHandle,
    message::SessionMessage,
};

use {
    crate::{
        ControlStream,
        error::ControlStreamError,
        transport::{Connection, error::DecodeError},
        types::message::ControlMessage,
    },
    error::{ctx, hs_ctx},
    snafu::ResultExt,
    tokio::sync::mpsc,
    tracing::error,
};

pub struct Session {
    transport: Connection,
    control_stream: ControlStream,
    handle: SessionHandle,
    // rx: Arc<RwLock<Receiver<SessionMessage>>>,
    // tx: Arc<Sender<SessionMessage>>,
}

impl Session {
    pub fn spawn(transport: Connection, control_stream: ControlStream) -> SessionHandle {
        let (session_tx, handler_rx) = mpsc::channel(10);
        let (handler_tx, session_rx) = mpsc::channel(10);

        let this = Self {
            transport,
            control_stream,
            handle: SessionHandle::new(session_tx, session_rx),
        };

        tokio::spawn(async move { this.handle().await });

        SessionHandle::new(handler_tx, handler_rx)
    }

    async fn handle(&self) {
        // TODO handle loop
        loop {
            // TODO tokio select for
            //  [ ] recv ControlMessage -> then handle
            //  [ ] recv SessionMessage from Relay
            //  [ ] recv Data on Unidirectional Streams
            // let t = self.transport.accept_uni().await
            tokio::select! {
                msg = self.control_stream.recv() => {
                    if let Err(err) = self.handle_control_messages(msg).await {
                        let _ = match err {
                            SessionError::ControlStream { source: ControlStreamError::Recv { source: DecodeError::EndOfStream } } => continue,
                            err => self.handle.send(SessionMessage::Error(err)).await,
                        };
                    }
                },
                uni = self.transport.accept_uni() => println!("accepted uni stream"),
                bidi = self.transport.accept_bi() => todo!("protocol violation: second bidi stream"),
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    async fn handle_control_messages(
        &self,
        control_message: Result<ControlMessage, ControlStreamError>,
    ) -> Result<(), SessionError> {
        let control_message = control_message.context(ctx::ControlStreamSnafu)?;

        match control_message {
            ControlMessage::ClientSetup(_client_setup) => todo!("error"),
            ControlMessage::Fetch(fetch) => todo!("fetch"),
            ControlMessage::FetchCancel(fetch_cancel) => todo!("fetch cancel"),
            ControlMessage::FetchError(fetch_error) => todo!("fetch error"),
            ControlMessage::FetchOk(fetch_ok) => todo!("fetch ok"),
            ControlMessage::GoAway(goaway) => todo!("goaway"),
            ControlMessage::MaxRequestId(max_request_id) => todo!("max request id"),
            ControlMessage::Publish(publish) => todo!("publish"),
            ControlMessage::PublishDone(publish_done) => todo!("publish done"),
            ControlMessage::PublishError(publish_error) => todo!("publish error"),
            ControlMessage::PublishNamespace(publish_namespace) => todo!("publish namespace"),
            ControlMessage::PublishNamespaceCancel(publish_namespace_cancel) => {
                todo!("publish namespace cancel")
            }
            ControlMessage::PublishNamespaceDone(publish_namespace_done) => {
                todo!("publish namespace done")
            }
            ControlMessage::PublishNamespaceError(publish_namespace_error) => {
                todo!("publish namespace error")
            }
            ControlMessage::PublishNamespaceOk(publish_namespace_ok) => {
                todo!("publish namespace ok")
            }
            ControlMessage::PublishOk(publish_ok) => todo!("publish ok"),
            ControlMessage::RequestsBlocked(requests_blocked) => todo!("requests blocked"),
            ControlMessage::ServerSetup(_server_setup) => todo!("error"),
            ControlMessage::Subscribe(subscribe) => {
                self.handle
                    .send(SessionMessage::Subscribe {
                        namespace: subscribe.namespace.clone(),
                        name: subscribe.name.clone(),
                    })
                    .await
                    .map_err(|err| {
                        error!(%err, "failed to send message on session handle");
                        SessionError::SessionHandle
                    })?;
                // TODO anything else that needs to be done?
            }
            ControlMessage::SubscribeError(subscribe_error) => todo!("subscribe error"),
            ControlMessage::SubscribeNamespace(subscribe_namespace) => todo!("subscribe namespace"),
            ControlMessage::SubscribeNamespaceError(subscribe_namespace_error) => {
                todo!("subscribe namespace error")
            }
            ControlMessage::SubscribeNamespaceOk(subscribe_namespace_ok) => {
                todo!("subscribe namespace ok")
            }
            ControlMessage::SubscribeOk(subscribe_ok) => todo!("subscribe ok"),
            ControlMessage::SubscribeUpdate(subscribe_update) => todo!("subscribe update"),
            ControlMessage::TrackStatus(track_status) => todo!("track status"),
            ControlMessage::TrackStatusError(track_status_error) => todo!("track status error"),
            ControlMessage::TrackStatusOk(track_status_ok) => todo!("track status ok"),
            ControlMessage::Unsubscribe(unsubscribe) => todo!("unsubscribe"),
            ControlMessage::UnsubscribeNamespace(unsubscribe_namespace) => {
                todo!("unsubscribe namespace")
            }
        }
        Ok(())
    }
}
