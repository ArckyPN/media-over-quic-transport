use crate::types::{Name, Namespace};

use super::SessionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionMessage {
    Error(SessionError),
    Subscribe { namespace: Namespace, name: Name },
}
