//! Frontend view selection for bulk approval release.
codetether_approval_view::declare_frontend_approval_release!(
    crate::tui::app::state::App,
    crate::tui::app::input::approval_command::host::Native,
);
#[cfg(test)]
#[path = "approval_command_release_tests.rs"]
mod tests;
