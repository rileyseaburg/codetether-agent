//! Image-attachment command behavior independent of a concrete TUI.
//!
//! Implement [`View`] for a pending attachment queue and use [`attach`] after
//! command argument trimming. [`Presentation`] preserves chat versus status-only output.
mod command;
mod view;
pub use command::attach;
pub use view::{Presentation, View};
