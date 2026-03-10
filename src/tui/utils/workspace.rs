pub struct WorkspaceSnapshot {
    pub root_display: String,
    pub git_branch: Option<String>,
    pub git_dirty_files: Vec<String>,
    pub entries: Vec<crate::tui::models::WorkspaceEntry>,
    pub captured_at: std::time::Instant,
}

pub struct WorkspaceEntry {
    pub name: String,
    pub kind: crate::tui::models::WorkspaceEntryKind,
}
