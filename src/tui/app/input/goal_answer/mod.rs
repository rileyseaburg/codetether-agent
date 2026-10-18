//! Pause goal work, answer user questions without tools, then await acceptance.

mod answer;
pub(crate) mod commands;
#[cfg(test)]
mod continuation;
mod delivery_host;
pub(crate) mod editor;
mod input_host;
mod review_host;
mod submit;

pub(crate) use answer::deliver;
pub(super) use submit::intercept;
