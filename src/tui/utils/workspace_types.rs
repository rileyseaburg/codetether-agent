//! Workspace snapshot types used by the webview sidebar.
//!
//! These lightweight data structures describe a point-in-time view of a project
//! directory for TUI and webview rendering. Collection logic lives in the
//! workspace scanning helpers; this module only defines the values those helpers
//! produce.

/// Category of an entry shown in a workspace snapshot.
///
/// The kind lets renderers choose appropriate icons, grouping, or styling
/// without inspecting the filesystem again.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::utils::workspace_types::WorkspaceEntryKind;
///
/// assert_eq!(WorkspaceEntryKind::File, WorkspaceEntryKind::File);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceEntryKind {
    /// A regular file or file-like item in the workspace listing.
    File,
    /// A directory that can contain additional workspace entries.
    Directory,
}

/// Single file or directory displayed in a workspace snapshot.
///
/// Each entry stores the display name collected from the filesystem and its
/// already-detected [`WorkspaceEntryKind`]. Paths are intentionally not stored
/// here so sidebar rendering can remain compact and relative to the snapshot
/// root.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::utils::workspace_types::{WorkspaceEntry, WorkspaceEntryKind};
///
/// let entry = WorkspaceEntry {
///     name: "src".to_string(),
///     kind: WorkspaceEntryKind::Directory,
/// };
/// assert_eq!(entry.name, "src");
/// ```
#[derive(Debug, Clone)]
pub struct WorkspaceEntry {
    /// Display name of the file or directory relative to the scanned location.
    pub name: String,
    /// Whether this entry represents a file or directory.
    pub kind: WorkspaceEntryKind,
}

/// Point-in-time workspace summary used by sidebar renderers.
///
/// A snapshot combines a human-readable root path, optional git branch name,
/// count of dirty files, visible workspace entries, and a capture timestamp.
/// It is designed to be cheap to clone and safe to render without additional
/// filesystem or git access.
///
/// # Invariants
///
/// `git_dirty_files` is a count supplied by the git detection helper and should
/// represent the number of changed paths known at capture time. `captured_at` is
/// display text, not a parseable timestamp contract.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::utils::workspace_types::WorkspaceSnapshot;
///
/// let snapshot = WorkspaceSnapshot::default();
/// assert!(snapshot.entries.is_empty());
/// assert_eq!(snapshot.git_dirty_files, 0);
/// ```
#[derive(Debug, Clone, Default)]
pub struct WorkspaceSnapshot {
    /// Human-readable workspace root path.
    pub root_display: String,
    /// Current git branch name when the root is inside a repository.
    pub git_branch: Option<String>,
    /// Number of dirty files detected in the repository at capture time.
    pub git_dirty_files: usize,
    /// Bounded list of files and directories selected for sidebar display.
    pub entries: Vec<WorkspaceEntry>,
    /// Local display time when the snapshot was captured.
    pub captured_at: String,
}
