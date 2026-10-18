//! Frontend storage selection and shared text-paste helpers.
#[cfg(not(test))]
pub use codetether_paste_buffer::{PendingTextPaste, expand_paste_placeholders};
codetether_paste_buffer::declare_frontend_text_paste!(crate::tui::app::state::AppState);

#[cfg(test)]
include!("pasted_text_test_imports.rs");
