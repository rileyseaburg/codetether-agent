//! Bind bulk approval release to native UI state and integration coverage.
use crate::tui::app::state::App;
pub(crate) fn all(app: &mut App) -> Result<usize, String> {
    codetether_approval_view::command::release_all(&mut super::host::Native(app))
}
#[cfg(test)]
#[path = "approval_command_release_tests.rs"]
mod tests;
