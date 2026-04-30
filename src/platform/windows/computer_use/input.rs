//! Win32 input injection via SendInput — replaces PowerShell mouse_event/SendKeys.

mod chord;
mod click;
mod key;
mod parse;
mod scroll;
mod text;
mod vk_table;

pub use chord::send_chord;
pub use click::send_click;
pub use key::send_key;
pub use parse::parse_send_keys;
pub use scroll::send_scroll;
pub use text::send_text;
