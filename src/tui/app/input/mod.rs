//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem.

mod backspace;
mod base_branch;
mod bus;
mod char_input;
mod chat_helpers;
mod chat_spawn;
mod chat_spawn_task;
mod chat_steer_queue;
mod chat_submit;
mod chat_submit_dispatch;

// Re-exports so the event loop's auto-drain hook can submit a queued
// user message as a fresh turn without duplicating the dispatch logic.
pub(crate) use chat_helpers::push_user_messages;
pub(crate) use chat_submit_dispatch::dispatch_prompt;
mod enter;
pub(crate) mod image;
mod merge;
mod paste;
mod pr;
mod pr_body;
mod pr_command;
mod pr_description;
mod pr_helpers;
mod pr_title;
mod sessions;
mod tests_enter;
mod tests_paste;
mod tests_pr;
mod tests_steering;
mod tests_submit;
mod worktree;
mod worktree_result;

pub use backspace::handle_backspace;
pub use bus::{handle_bus_c, handle_bus_g, handle_bus_slash};
pub use char_input::handle_char;
pub use enter::dispatch_enter as handle_enter;
pub(crate) use image::attach_image_file;
pub use paste::handle_paste;
pub use sessions::handle_sessions_char;
