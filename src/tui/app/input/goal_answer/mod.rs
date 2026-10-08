//! Pause goal work, answer user questions without tools, then await acceptance.

mod answer;
pub(crate) mod commands;
mod continuation;
pub(crate) mod editor;
mod submit;

pub(crate) use answer::deliver;
pub(super) use submit::intercept;
