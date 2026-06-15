//! `!command` shell prefix support for the chat composer.
//!
//! Typing `!<command>` in the chat input runs the command in the
//! workspace shell instead of sending it to the model.

mod command;
mod exec;
#[cfg(test)]
mod tests;
#[cfg(test)]
#[path = "tests_shell_kind.rs"]
mod tests_shell_kind;
mod truncate;

pub(super) use command::run;
