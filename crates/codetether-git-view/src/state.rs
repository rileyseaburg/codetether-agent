//! State for the TUI git view.

use serde::{Deserialize, Serialize};

/// Mutable state backing the `/git` TUI view.
///
/// Captured on demand via [`crate::capture_git_state`] by running `git` subprocesses
/// inside the current workspace directory.
/// # Examples
/// ```
/// let state = codetether_git_view::GitViewState::default();
/// assert!(state.branch.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitViewState {
    /// Current branch name (or `None` if not a git repo).
    pub branch: Option<String>,
    /// Number of dirty (modified/untracked) files.
    pub dirty_files: usize,
    /// `git log --oneline` lines (limited to last 20).
    pub log_lines: Vec<String>,
    /// `git diff --stat` output (staged + unstaged).
    pub diff_stat: String,
    /// `git branch -v` lines showing local branches.
    pub branches: Vec<String>,
    /// Capture timestamp.
    pub captured_at: String,
}
