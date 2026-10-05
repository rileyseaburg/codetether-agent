//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem.

pub(crate) mod approval_command;
mod approval_feedback;
mod backspace;
mod base_branch;
mod bus;
mod char_input;
mod chat_helpers;
mod chat_runtime_submit;
mod chat_spawn;
mod chat_steer;
mod chat_submit;
pub(crate) mod chat_submit_dispatch;
mod chat_submit_finish;
mod chat_submit_slash;
mod codex_parity_command;
mod continue_command;
pub(crate) mod goal_answer;

// Re-exports so the event loop's auto-drain hook can submit a queued
// user message as a fresh turn without duplicating the dispatch logic.
mod enter;
mod enter_subagents;
pub(crate) mod image;
mod image_data_paste;
mod image_data_url;
mod image_file;
mod image_mime;
mod image_sidecar_recover;
pub(crate) mod mention_route;
mod merge;
mod model_apply;
mod paste;
mod paste_expand_raw;
pub(crate) mod pasted_text;
mod pr;
mod pr_body;
mod pr_command;
mod pr_description;
mod pr_helpers;
mod pr_request;
mod pr_title;
pub(crate) mod sessions;
pub(crate) mod shell_bg;
pub(crate) mod worktree;
pub(crate) mod worktree_result;

#[cfg(test)]
mod tests_all;

mod chat_submit_history;
mod exports;
pub use exports::*;
