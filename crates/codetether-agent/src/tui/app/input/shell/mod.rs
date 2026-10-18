//! `!command` shell prefix support for the chat composer.
//!
//! Typing `!<command>` in the chat input runs the command in the
//! workspace shell instead of sending it to the model. Execution happens
//! on a background task (see [`crate::tui::app::input::shell_bg`]) so the
//! TUI stays responsive; results are drained into the transcript by the
//! event loop.

mod command;
#[cfg(test)]
mod tests;
#[cfg(test)]
#[path = "tests_shell_kind.rs"]
mod tests_shell_kind;
#[cfg(test)]
mod tests_support;

pub(super) use command::run;
