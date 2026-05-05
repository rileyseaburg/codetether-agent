//! Capture git status for the TUI git view.

use std::path::Path;

use crate::tui::app::state::git_state::GitViewState;

/// Run git commands to populate a [`GitViewState`] snapshot.
pub fn capture_git_state(cwd: &Path) -> GitViewState {
    let (branch, dirty_files) =
        crate::tui::utils::workspace_helpers::detect_git_status(cwd);
    let log_lines = super::git_log::capture_log(cwd);
    let diff_stat = super::git_diff::capture_diff_stat(cwd);
    let branches = super::git_branch::capture_branches(cwd);
    let captured_at = chrono::Local::now().format("%H:%M:%S").to_string();

    GitViewState {
        branch,
        dirty_files,
        log_lines,
        diff_stat,
        branches,
        captured_at,
    }
}
