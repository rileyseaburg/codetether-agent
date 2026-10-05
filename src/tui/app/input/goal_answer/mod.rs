//! Pause goal work, answer user questions without tools, then await acceptance.

mod answer;
mod submit;

pub(crate) use answer::deliver;
pub(super) use submit::intercept;
