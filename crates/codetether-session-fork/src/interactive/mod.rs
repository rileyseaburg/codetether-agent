//! Interactive trailing-turn fork commands with preserved TUI semantics.
//!
//! Implement [`Session`] and [`View`], then call [`execute`]. The command saves
//! the parent, creates and persists a new identity, then switches the view.
mod copy;
mod creation;
mod cuts;
mod execute;
mod metadata;
mod notice;
mod session;
mod view;
pub use execute::execute;
pub use session::Session;
pub use view::View;
