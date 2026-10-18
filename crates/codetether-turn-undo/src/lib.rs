//! Synchronized session and displayed-chat history truncation.
//!
//! [`truncate_last_turns`] removes complete user turns from both histories,
//! bounded by the smaller user-message count. Session page metadata follows
//! the retained session messages. [`command::execute`] coordinates persistence
//! and UI refresh through caller-supplied session and view interfaces.
pub mod command;
mod turn_undo;
mod turn_undo_count;
mod turn_undo_index;
pub use turn_undo::truncate_last_turns;
#[cfg(test)]
mod turn_undo_mismatch_tests;
#[cfg(test)]
mod turn_undo_tests;
