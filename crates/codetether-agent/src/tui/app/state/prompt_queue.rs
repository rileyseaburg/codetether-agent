//! Shared prompt queue UI binding using the native preview formatter.
codetether_session_steering::define_app_prompt_queue!(
    crate::tui::app::state::App,
    crate::tui::app::text::truncate_preview
);
pub use codetether_session_steering::follow_up::{has_pending, take};
