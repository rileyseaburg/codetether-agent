//! Input-handler API exports, separate from module registration.

pub use super::bus::{handle_bus_c, handle_bus_g, handle_bus_slash};
pub(crate) use super::enter::dispatch_enter as handle_enter;
#[cfg(not(test))]
pub(crate) use super::image::attach_image_command;
pub use super::paste::{handle_paste, paste_into_chat};
pub use super::sessions::handle_sessions_char;
pub use super::text_host::handle_backspace;
pub use super::text_host::handle_char;

#[cfg(test)]
include!("exports_test_imports.rs");
