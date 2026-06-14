//! `!command` shell prefix support for the chat composer.
//!
//! Typing `!<command>` in the chat input runs the command in the
//! workspace shell instead of sending it to the model.

mod command;
mod exec;
#[cfg(test)]
mod tests;
mod truncate;

pub(super) use command::run;
