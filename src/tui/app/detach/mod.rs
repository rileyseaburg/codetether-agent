//! `/detach` — snapshot the current TUI thread into a backgrounded sub-agent.
//!
//! The TUI stays the single process. `/detach` copies the current
//! conversation into a [`SpawnedAgent`](crate::tui::app::state::SpawnedAgent)
//! that runs in the background, leaving the active thread intact.
//!
//! - [`command`] handles the `/detach` slash command.
//! - [`child`] builds the persisted point-in-time session copy.

pub mod child;
pub mod command;
mod name;
mod register;

pub use command::handle_detach_command;
