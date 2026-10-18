//! Capture git status for the TUI git view.

use std::path::Path;

use crate::GitViewState;

/// Run git commands to populate a [`GitViewState`] snapshot.
/// # Arguments
/// * `cwd` - Workspace directory.
/// # Returns
/// Captured branch, status, logs, diff summary and timestamp.
/// # Examples
/// ```no_run
/// let _state = codetether_git_view::capture_git_state(std::path::Path::new("."));
/// ```
pub fn capture_git_state(cwd: &Path) -> GitViewState {
    let (branch, dirty_files) = crate::detect_git_status(cwd);
    let log_lines = crate::log::capture_log(cwd);
    let diff_stat = crate::diff::capture_diff_stat(cwd);
    let branches = crate::branches::capture_branches(cwd);
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
