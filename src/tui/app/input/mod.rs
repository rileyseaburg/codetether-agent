//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem.

pub(crate) mod approval_command;
mod approval_feedback;
mod backspace;
use codetether_git_view as base_branch;
mod bus;
mod char_input;
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
use codetether_image as image_data_url;
use codetether_image as image_file;
use codetether_image as image_mime;
mod image_sidecar_recover;
pub(crate) use codetether_chat_submit::mention as mention_route;
mod model_apply;
mod paste;
pub(crate) mod pasted_text;
use codetether_github_pr::worktree as pr;
use codetether_github_pr::worktree::body as pr_body;
use codetether_github_pr::worktree::command as pr_command;
use codetether_github_pr::worktree::description as pr_description;
use codetether_github_pr::worktree::helpers as pr_helpers;
use codetether_github_pr::worktree::request as pr_request;
use codetether_github_pr::worktree::title as pr_title;
pub(crate) mod sessions;
pub(crate) mod shell_bg;
pub(crate) mod worktree;
pub(crate) mod worktree_result;

#[cfg(test)]
mod tests_all;

mod chat_submit_history;
mod exports;
pub use exports::*;
