//! Keyboard, mouse and paste event dispatch for the TUI.

mod alt_scroll;
mod approval_key;
#[cfg(test)]
mod approval_key_tests;
mod click_open;
mod click_path;
mod clipboard;
mod copy_reply;
mod copy_transcript;
mod ctrl_c;
mod editor_click;
mod editor_key;
mod editor_lsp;
mod editor_lsp_hover;
mod editor_lsp_key;
mod editor_lsp_nav;
mod editor_lsp_retry;
mod event_dispatch;
mod fuzzy_find_key;
mod goal_prompt_key;
mod interlude_key;
mod interrupt_key;
#[cfg(test)]
mod interrupt_key_tests;
mod key_repeat;
mod keybinds;
mod keyboard;
mod mode_keys;
mod model_picker_key;
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
mod shared_file_open;
mod side_question;
mod tab_keys;
mod tests;
pub(crate) mod voice;

use keybinds::handle_unmodified_key;
use keyboard::handle_ctrl_key;

pub(crate) use event_dispatch::handle_event;
pub use mouse::handle_mouse_event;
pub use paste::handle_paste_event;
pub(crate) use voice::drain_voice_transcription;
