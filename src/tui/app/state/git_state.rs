//! State for the TUI git view.

use serde::{Deserialize, Serialize};

/// Mutable state backing the `/git` TUI view.
///
/// Captured on demand via [`Self::capture`] by running `git` subprocesses
/// inside the current workspace directory.
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
