//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem.

pub(crate) mod approval_command;
mod backspace;
mod base_branch;
mod bus;
mod char_input;
mod chat_helpers;
mod chat_spawn;
mod chat_submit;
pub(crate) mod chat_submit_dispatch;
mod chat_submit_finish;
mod chat_submit_slash;
mod codex_parity_command;
mod continue_command;

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
mod sessions;
pub(crate) mod shell_bg;
pub(crate) mod worktree;
pub(crate) mod worktree_result;

#[cfg(test)]
mod tests_all;

pub use backspace::handle_backspace;
pub use bus::{handle_bus_c, handle_bus_g, handle_bus_slash};
pub use char_input::handle_char;
pub(crate) use enter::dispatch_enter as handle_enter;
pub(crate) use image::attach_image_file;
pub(crate) use image_data_paste::try_attach_data_url;
pub use paste::{handle_paste, paste_into_chat};
pub use sessions::handle_sessions_char;
