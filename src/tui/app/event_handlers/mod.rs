//! Keyboard, mouse and paste event dispatch for the TUI.

mod alt_scroll;
mod clipboard;
mod copy_reply;
mod copy_transcript;
mod ctrl_c;
mod event_dispatch;
mod keybinds;
mod keyboard;
mod mode_keys;
mod mouse;
mod okr;
mod okr_save;
mod overlay_scroll;
mod paste;
mod paste_burst;
#[cfg(test)]
mod paste_burst_tests;
mod scroll_down;
mod scroll_up;
mod tests;
pub(crate) mod voice;

use keybinds::handle_unmodified_key;
use keyboard::handle_ctrl_key;

pub(crate) use event_dispatch::handle_event;
pub use mouse::handle_mouse_event;
pub use paste::handle_paste_event;
pub(crate) use voice::drain_voice_transcription;
