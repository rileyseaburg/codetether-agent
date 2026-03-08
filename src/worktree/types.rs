//! Worktree data structures
use std::path::PathBuf;

/// Worktree information
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub active: bool,
}

/// Merge operation result
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub success: bool,
    pub aborted: bool,
    pub conflicts: Vec<String>,
    pub conflict_diffs: Vec<(String, String)>,
    pub files_changed: usize,
    pub summary: String,
}
