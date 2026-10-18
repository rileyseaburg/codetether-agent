//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem.

pub(crate) mod approval_command;
mod approval_feedback;
mod bus;
mod chat_helpers;
mod chat_spawn;
mod chat_steer;
mod chat_submit;
pub(crate) mod chat_submit_dispatch;
mod chat_submit_finish;
mod chat_submit_slash;
mod codex_parity_command;
mod continue_command;
pub(crate) mod goal_answer;
mod text_host;

// Re-exports so the event loop's auto-drain hook can submit a queued
// user message as a fresh turn without duplicating the dispatch logic.
mod enter;
mod enter_subagents;
pub(crate) mod image;
mod image_data_paste;
mod image_sidecar_recover;
pub(crate) use codetether_chat_submit::mention as mention_route;
mod paste;
pub(crate) mod pasted_text;
pub(crate) mod sessions;
pub(crate) mod shell_bg;
pub(crate) mod worktree;
pub(crate) mod worktree_result;

#[cfg(test)]
mod tests_all;

mod chat_submit_history;
mod exports;
pub use exports::*;

#[cfg(test)]
include!("mod_test_imports.rs");
