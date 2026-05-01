//! Win32 input injection via SendInput — replaces PowerShell mouse_event/SendKeys.

mod chord;
mod click;
mod double_click;
mod drag;
mod key;
mod parse;
mod right_click;
mod scroll;
mod text;
mod vk_table;

pub use chord::send_chord;
pub use click::send_click;
pub use double_click::send_double_click;
pub use drag::send_drag;
pub use key::send_key;
pub use parse::parse_send_keys;
pub use right_click::send_right_click;
pub use scroll::send_scroll;
pub use text::send_text;
