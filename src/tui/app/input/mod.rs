//! Text input, Enter, backspace and paste handlers for TUI views.
//!
//! Each handler inspects the active [`ViewMode`] and delegates
//! to the appropriate subsystem — chat submission, session
//! switching, bus filtering, model picker, file picker, etc.
//!
//! # Examples
//!
//! ```ignore
//! handle_enter(&mut app, cwd, &mut session, &reg,
//!     &bridge, &tx, &rtx).await;
//! ```

mod backspace;
mod base_branch;
mod bus;
mod char_input;
mod chat_helpers;
mod chat_spawn;
mod chat_submit;
mod enter;
pub(crate) mod image;
mod merge;
mod paste;
mod pr;
mod pr_command;
mod pr_helpers;
mod sessions;
#[cfg(test)]
mod tests;
mod worktree;
mod worktree_result;

pub use backspace::handle_backspace;
pub use bus::{handle_bus_c, handle_bus_g, handle_bus_slash};
pub use char_input::handle_char;
pub use enter::dispatch_enter as handle_enter;
pub(crate) use image::attach_image_file;
pub use paste::handle_paste;
pub use sessions::handle_sessions_char;
