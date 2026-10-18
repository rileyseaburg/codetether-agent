//! Interactive undo validation, mutation, persistence and presentation.
//!
//! Implement [`Session`] and [`View`] to bind existing storage and UI state,
//! then call [`execute`] with the raw command arguments.
mod execute;
mod notice;
mod persistence;
mod session;
mod view;
pub use execute::execute;
pub use session::Session;
pub use view::View;
