//! File attachment commands and composer updates through client bindings.
//!
//! Implement [`View`] and use [`command`] for a trimmed command argument or
//! [`attach`] for a selected file path. File snippets keep their configured limit.
mod attach;
mod command;
mod composer;
mod notice;
mod view;
pub use attach::attach;
pub use command::command;
pub use view::View;
