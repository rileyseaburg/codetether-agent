//! DOM action routing.

mod motion;
mod read;
mod write;

/// Motion-oriented DOM actions.
pub(in crate::tool::browserctl) use motion::{blur, focus, hover, scroll};
/// DOM read actions.
pub(in crate::tool::browserctl) use read::{html, text};
/// DOM write actions.
pub(in crate::tool::browserctl) use write::{click, fill, press, type_text};
