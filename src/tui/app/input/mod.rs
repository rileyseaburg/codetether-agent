//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem.

pub(crate) mod approval_command;
#[cfg(test)]
mod approval_command_deny_tests;
#[cfg(test)]
mod approval_command_tests;
mod backspace;
mod base_branch;
mod bus;
mod char_input;
mod chat_helpers;
mod chat_spawn;
mod chat_submit;
pub(crate) mod chat_submit_dispatch;
mod chat_submit_slash;
mod codex_parity_command;
mod continue_command;
mod forage_offer;

// Re-exports so the event loop's auto-drain hook can submit a queued
// user message as a fresh turn without duplicating the dispatch logic.
mod enter;
pub(crate) mod image;
mod image_data_paste;
mod image_data_url;
mod image_file;
mod image_mime;
mod image_tests;
mod mention_route;
mod merge;
mod paste;
pub(crate) mod pasted_text;
mod pr;
mod pr_body;
mod pr_command;
mod pr_command_tests;
mod pr_description;
mod pr_helpers;
mod pr_request;
mod pr_title;
mod sessions;
pub(crate) mod shell_bg;
mod tests_enter;
mod tests_image_paste;
mod tests_paste;
mod tests_pr;
mod tests_submit;
pub(crate) mod worktree;
pub(crate) mod worktree_result;

pub use backspace::handle_backspace;
pub use bus::{handle_bus_c, handle_bus_g, handle_bus_slash};
pub use char_input::handle_char;
pub(crate) use enter::dispatch_enter as handle_enter;
pub(crate) use image::attach_image_file;
pub(crate) use image_data_paste::try_attach_data_url;
pub use paste::handle_paste;
pub use sessions::handle_sessions_char;
