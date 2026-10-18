//! Frontend resource selection for pending-approval key actions.
codetether_approval_view::declare_frontend_approval_keys!(
    crate::tui::app::state::App,
    crate::tui::models::ViewMode,
    codetether_editor::FileBuffer,
    codetether_approval_edit::ApprovalEditSession,
    crossterm::event::KeyEvent,
    crossterm::event::KeyCode;
    crate::tui::app::input::approval_command::run,
);
#[cfg(test)]
#[path = "approval_navigation_key_tests.rs"]
mod navigation_tests;
#[cfg(test)]
#[path = "approval_scroll_limit_tests.rs"]
mod scroll_limit_tests;

#[cfg(test)]
#[path = "approval_feedback_key_tests.rs"]
mod feedback_tests;
#[cfg(test)]
#[path = "approval_hidden_key_tests.rs"]
mod hidden_tests;
#[cfg(test)]
#[path = "approval_scroll_key_tests.rs"]
mod scroll_tests;
