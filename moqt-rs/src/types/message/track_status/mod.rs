mod error;
mod ok;

pub use error::TrackStatusError;
pub use ok::TrackStatusOk;

use crate::types::message::Subscribe;

/// See [Subscribe] for details.
pub type TrackStatus = Subscribe;
