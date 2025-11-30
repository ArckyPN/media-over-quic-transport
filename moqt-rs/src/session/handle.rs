use {
    super::{SessionHandleError, SessionMessage, hs_ctx},
    snafu::ResultExt,
    std::sync::Arc,
    tokio::sync::{
        RwLock,
        mpsc::{Receiver, Sender, error::TryRecvError},
    },
};

#[derive(Debug)]
pub struct SessionHandle {
    rx: Arc<RwLock<Receiver<SessionMessage>>>,
    tx: Arc<Sender<SessionMessage>>,
}

impl SessionHandle {
    pub fn new(tx: Sender<SessionMessage>, rx: Receiver<SessionMessage>) -> Self {
        Self {
            rx: Arc::new(RwLock::new(rx)),
            tx: Arc::new(tx),
        }
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn recv(&self) -> Result<Option<SessionMessage>, SessionHandleError> {
        match self.rx.write().await.try_recv() {
            Ok(msg) => Ok(Some(msg)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(source @ TryRecvError::Disconnected) => {
                Err(SessionHandleError::ChannelClosed { source })
            }
        }
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn send(&self, msg: SessionMessage) -> Result<(), SessionHandleError> {
        self.tx.send(msg).await.context(hs_ctx::ChannelSendSnafu)
    }
}
