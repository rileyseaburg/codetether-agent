//! Actions for files referenced by agents in the chat transcript.
//!
//! Keyed off the most-recent file reference in the transcript — either a
//! shared `MessageType::File` message or a file-oriented tool call
//! (`read`/`write`/`edit`/…). `Ctrl+E` shows the file's git diff in `/git`;
//! `Ctrl+F` opens it in the standard read-only file viewer.

mod actions;
mod extract;
mod keys;
mod resolve;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_negative;

pub(crate) use keys::ctrl_key;
