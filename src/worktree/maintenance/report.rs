use super::WorktreeCleanupEntry;
use serde::Serialize;

/// Complete preview or result of safe repository worktree cleanup.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::worktree::maintenance::WorktreeCleanupReport;
///
/// let report = WorktreeCleanupReport {
///     base: "main".into(), applied: false, entries: Vec::new(),
/// };
/// assert!(!report.applied);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct WorktreeCleanupReport {
    /// Branch or commit used for merged-commit ancestry checks.
    pub base: String,
    /// Whether cleanup was applied rather than previewed.
    pub applied: bool,
    /// Decisions and outcomes for all worktrees within the selected roots.
    pub entries: Vec<WorktreeCleanupEntry>,
}
