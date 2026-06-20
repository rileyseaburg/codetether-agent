//! Background execution for the `!command` chat shell prefix.
//!
//! The shell command runs on a detached Tokio task and reports its result
//! through an unbounded channel that the event loop drains on each tick. This
//! keeps the TUI responsive (rendering, input, Ctrl+C) while long-running
//! commands execute, instead of blocking the event handler inline.

mod drain;
mod event;
mod run;
mod spawn;
pub(crate) mod truncate;

pub(crate) use drain::drain_shell_events;
pub use event::ShellEvent;
pub(crate) use spawn::spawn_shell_command;
