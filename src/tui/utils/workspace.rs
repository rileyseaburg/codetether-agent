//! Workspace snapshot: directory listing + git status for the webview sidebar.
//!
//! Captured synchronously on demand. Types live in [`super::workspace_types`],
//! FS scanning in [`super::workspace_entries`], git queries in
//! [`super::workspace_helpers`].

use std::path::Path;

use super::workspace_entries::{collect_entries, sort_entries};
use super::workspace_helpers::detect_git_status;

pub use super::workspace_types::{WorkspaceEntry, WorkspaceEntryKind, WorkspaceSnapshot};

impl WorkspaceSnapshot {
    /// Capture the current state of `root` truncated to `max_entries`.
    pub fn capture(root: &Path, max_entries: usize) -> Self {
        let mut entries = collect_entries(root);
        sort_entries(&mut entries);
        entries.truncate(max_entries);
        let (git_branch, git_dirty_files) = detect_git_status(root);
        Self {
            root_display: root.to_string_lossy().to_string(),
            git_branch,
            git_dirty_files,
            entries,
            captured_at: chrono::Local::now().format("%H:%M:%S").to_string(),
        }
    }
}
