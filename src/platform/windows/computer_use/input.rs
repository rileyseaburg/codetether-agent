//! Win32 input injection via SendInput — replaces PowerShell mouse_event/SendKeys.

mod click;
mod key;
mod scroll;
mod text;

pub use click::send_click;
pub use key::send_key;
pub use scroll::send_scroll;
pub use text::send_text;
