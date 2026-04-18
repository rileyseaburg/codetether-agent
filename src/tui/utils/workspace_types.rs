//! Workspace snapshot types used by the webview sidebar.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceEntryKind {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct WorkspaceEntry {
    pub name: String,
    pub kind: WorkspaceEntryKind,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceSnapshot {
    pub root_display: String,
    pub git_branch: Option<String>,
    pub git_dirty_files: usize,
    pub entries: Vec<WorkspaceEntry>,
    pub captured_at: String,
}
