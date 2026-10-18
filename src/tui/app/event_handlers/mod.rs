//! Keyboard, mouse and paste event dispatch for the TUI.

mod alt_scroll;
mod answer_review_key;
mod approval_key;
#[cfg(test)]
mod approval_key_tests;
mod click_open;
mod ctrl_c;
mod editor_click;
mod editor_key;
mod editor_lsp;
mod editor_lsp_hover;
mod editor_lsp_key;
mod event_dispatch;
mod event_priority;
mod fuzzy_find_key;
mod goal_prompt_key;
mod interlude_key;
mod interrupt_host;
mod interrupt_key;
#[cfg(test)]
mod interrupt_key_tests;
mod keybinds;
mod keyboard;
mod mode_keys;
mod model_picker_key;
mod mouse;
mod okr;
mod paste;
mod paste_burst;
mod shared_file_open;
mod side_question;
mod tab_keys;
mod tests;
pub(crate) mod voice;

mod exports;
pub(crate) use exports::{drain_voice_transcription, handle_event};
pub use exports::{handle_mouse_event, handle_paste_event};

codetether_chat_clipboard::declare_frontend_chat_clipboard!(
    crate::tui::app::state::App,
    codetether_chat_model::ChatMessage,
    codetether_image::ImageAttachment;
    crate::tui::constants::SCROLL_BOTTOM,
    crate::tui::app::input::paste_into_chat,
    codetether_chat_transcript::build_transcript,
);
