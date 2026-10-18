//! Approval-backed source editor orchestration.

#[cfg(test)]
use codetether_approval_edit::interactive as finish;
#[cfg(test)]
#[path = "key_tests.rs"]
mod key_tests;
#[cfg(test)]
#[path = "finish_tests.rs"]
mod tests;

pub(super) use key::handle;

codetether_approval_edit::declare_frontend_approval_editor!(
    pub(in crate::tui::app::event_handlers);
    crate::tui::app::state::App,
    crate::tui::models::ViewMode,
    codetether_editor::FileBuffer,
    codetether_approval_edit::ApprovalEditSession,
    codetether_editor::EditorInput,
);
