//! Interoperability traits for Bevy components

#![warn(missing_docs)]

pub mod position;

/// Module for convenient imports. Use with `use seldom_interop::prelude::*;`.
pub mod prelude {
    pub(crate) use bevy::prelude::*;

    pub use crate::position::{Position2, Position3};
}
