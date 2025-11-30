mod builder;
mod config;
mod error;

pub use {
    config::{Protocol, ServerConfig},
    error::ServerError,
};

use {
    crate::{
        Session, error::ControlStreamError, session::SessionHandle, transport::Endpoint,
        types::error_code::Termination,
    },
    core::net::SocketAddr,
    dashmap::DashMap,
    error::ctx,
    snafu::ResultExt,
    tracing::{error, info},
};

/// A Server is the MOQT **Relay**.
///
/// It is named Server to keep it consistent
/// with typical naming schemes.
/// TODO docs
#[derive(Debug)]
pub struct Server {
    transport: Endpoint,
    sessions: DashMap<SocketAddr, SessionHandle>,
    // TODO add Client connections to other Relays/(ControlTower?) to query them for Tracks this Relay doesn't know
}
// TODO add HTTP server for dashboard and outside control, like shutdown, etc.

impl Server {
    /// Launches the [Relay] making it run until
    /// termination or encountering an fatal error.
    #[tracing::instrument(skip(self))]
    pub async fn run(&self) {
        info!(
            addr = ?self.transport.local_address(),
            proto = self.transport.proto(),
            "Relay is running"
        );

        loop {
            tokio::select! {
                biased;
                res = self.accept_session() => {
                    match res {
                        Ok(_) => info!("accepted new session"),
                        Err(_) => error!("failed to accept new session")
                    }
                }
                // res = self.recv_session_messages() => {
                //     if let Err(_err) = res {
                //         todo!("what to do when server runs into an error? shouldn't happen, so panic?")
                //     }
                // }
                // TODO check sessions channels and handle them
            }
        }
    }

    #[tracing::instrument(skip(self), err)]
    async fn accept_session(&self) -> Result<(), ServerError> {
        let conn = self.transport.accept().await.context(ctx::EndpointSnafu)?;

        let control_stream = match crate::ControlStream::accept(&conn).await {
            Ok(cs) => cs,
            Err(source @ ControlStreamError::NoSupportedVersion { .. }) => {
                error!("unable to negotiate a version, dropping connection...");
                conn.close(Termination::VersionNegotiationFailed);
                return Err(ServerError::ControlStream { source });
            }
            Err(source) => {
                error!(%source, "failed to establish ControlStream, dropping connection...");
                return Err(ServerError::ControlStream { source });
            }
        };

        // TODO I need a way to poll the session to
        // [ ] exchange Messages, like
        //      [ ] forcing a Goaway to all/specific sessions
        //      [ ] collecting announced Tracks
        //      [ ] relaying Tracks to sessions
        // [ ] remove session and their associated announced/subscribed Tracks when they error
        // [ ] more?
        let addr = conn.remote_addr();
        let handle = Session::spawn(conn, control_stream);

        self.sessions.insert(addr, handle);
        Ok(())
    }

    async fn recv_session_messages(&self) -> Result<(), ServerError> {
        let mut remove = Vec::new();

        for session in &self.sessions {
            let (addr, session) = session.pair();
            let msg = match session.recv().await {
                Ok(Some(msg)) => msg,
                Ok(None) => continue,
                Err(err) => {
                    error!("session ran into an error: {}", err);
                    remove.push(addr.to_owned());
                    continue;
                }
            };

            todo!("handle session message: {:?}", msg);
        }
        Ok(())
    }
}
