//! DOM action routing.

mod motion;
mod read;
mod write;

/// Motion-oriented DOM actions.
pub(crate) use motion::{blur, focus, hover, scroll};
/// DOM read actions.
pub(crate) use read::{html, text};
/// DOM write actions.
pub(crate) use write::{click, fill, press, type_text};
