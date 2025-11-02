mod error;
mod ok;

pub use error::TrackStatusError;
pub use ok::TrackStatusOk;

use crate::types::message::Subscribe;

/// ## TrackStatus
///
/// See [Subscribe] for details.
pub type TrackStatus = Subscribe;
