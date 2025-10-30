pub mod error;

pub use error::Error;

pub use varint_core as core;

pub use varint_core::{Reader, VarInt, Writer};
pub use varint_derive::{VarInt, varint_enum, x};

#[doc(hidden)]
pub use funty;

#[doc(hidden)]
pub use snafu;
