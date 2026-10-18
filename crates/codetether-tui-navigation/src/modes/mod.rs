//! Mode-specific character, backspace and settings-key selection.
//!
//! [dispatch] selects [Action] and [Setting] operations through [Host] after
//! navigation and modifier shortcuts have been handled.
mod action;
mod host;
mod run;
pub use action::{Action, Setting};
pub use host::{Host, Input, State};
pub use run::dispatch;
