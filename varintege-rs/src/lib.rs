pub mod error;

pub use error::Error;

pub use varint_core as core;

pub use varint_core::{Parameter, Reader, VarInt, VarIntBytes, VarIntNumber, Writer};
pub use varint_derive::{VarInt, varint_enum, x};
// TODO moq feature
pub use varint_derive::draft_ref;

pub mod prelude {
    pub use crate::{VarInt, VarIntBytes, VarIntNumber, x};
}

#[doc(hidden)]
pub use funty;

#[doc(hidden)]
pub use snafu;
