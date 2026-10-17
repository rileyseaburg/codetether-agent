//! Win32 input injection via SendInput — replaces PowerShell mouse_event/SendKeys.

mod chord;
mod click;
mod cursor_move;
mod double_click;
mod drag;
mod drag_motion;
mod key;
mod modifier;
mod mouse_button;
mod mouse_event;
mod mouse_state;
mod parse;
mod right_click;
mod scroll;
mod text;
mod vk_table;

pub use chord::send_chord;
pub use click::send_click;
pub use cursor_move::move_cursor;
pub use double_click::send_double_click;
pub use drag::send_drag;
pub use key::send_key;
pub use modifier::{hold_modifiers, modifier_vks, release_modifiers};
pub use mouse_button::drag_button;
pub use mouse_event::send_mouse_button;
pub use mouse_state::{mouse_down, mouse_up};
pub use parse::parse_send_keys;
pub use right_click::send_right_click;
pub use scroll::send_scroll;
pub use text::send_text;
