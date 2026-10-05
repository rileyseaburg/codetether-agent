//! Input-handler API exports, separate from module registration.

pub use super::backspace::handle_backspace;
pub use super::bus::{handle_bus_c, handle_bus_g, handle_bus_slash};
pub use super::char_input::handle_char;
pub(crate) use super::enter::dispatch_enter as handle_enter;
pub(crate) use super::image::attach_image_file;
pub use super::paste::{handle_paste, paste_into_chat};
pub use super::sessions::handle_sessions_char;
