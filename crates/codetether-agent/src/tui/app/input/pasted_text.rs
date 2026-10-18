//! Frontend storage selection and shared text-paste helpers.
pub use codetether_paste_buffer::{
    PendingTextPaste, expand_paste_placeholders, format_size, should_summarize,
};
codetether_paste_buffer::declare_frontend_text_paste!(crate::tui::app::state::AppState);
