//! Workspace snapshot: directory listing + git status for the webview sidebar.
//!
//! Captured synchronously on demand. Types live in [`super::workspace_types`],
//! FS scanning in [`super::workspace_entries`], git queries in
//! [`super::workspace_helpers`].
//!
//! A snapshot is a point-in-time view of a workspace root. It combines a bounded
//! directory entry list with lightweight git metadata so the TUI and webview
//! sidebar can render project context without performing filesystem work during
//! drawing.

use std::path::Path;

use super::workspace_entries::{collect_entries, sort_entries};
use super::workspace_helpers::detect_git_status;

pub use super::workspace_types::{WorkspaceEntry, WorkspaceEntryKind, WorkspaceSnapshot};

impl WorkspaceSnapshot {
    /// Capture the current state of `root` truncated to `max_entries`.
    ///
    /// Entries are collected from the filesystem, sorted for stable display,
    /// truncated to the requested limit, and paired with git branch and dirty
    /// file information detected for the same root.
    ///
    /// # Parameters
    ///
    /// * `root` - Workspace directory to inspect.
    /// * `max_entries` - Maximum number of filesystem entries to retain in the
    ///   returned snapshot after sorting.
    ///
    /// # Returns
    ///
    /// Returns a [`WorkspaceSnapshot`] containing a display form of `root`,
    /// optional git metadata, the collected entries, and a local-time capture
    /// timestamp formatted as `HH:MM:SS`.
    ///
    /// # Side Effects
    ///
    /// Reads directory contents and may run git status detection through helper
    /// code. Filesystem or git errors are handled by the helper functions and do
    /// not propagate from this method.
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
