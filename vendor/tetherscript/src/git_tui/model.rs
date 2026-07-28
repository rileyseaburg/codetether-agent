//! Data model for the git panel.

/// One changed path reported by `git status --short --branch`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitEntry {
    /// Two-character porcelain status, such as ` M`, `M `, `??`, or `UU`.
    pub code: String,
    /// Changed path as reported by git.
    pub path: String,
    /// Normalized status bucket used by the TUI.
    pub kind: GitEntryKind,
}

/// Status bucket for a git entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitEntryKind {
    Staged,
    Unstaged,
    Untracked,
    Conflicted,
}

/// Summary shown by the TUI git panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitPanel {
    /// Branch/upstream line without the porcelain `## ` prefix.
    pub branch: String,
    /// Changed entries, preserving git status order.
    pub entries: Vec<GitEntry>,
}

impl GitPanel {
    /// Returns true when the repository has no visible changes.
    pub fn is_clean(&self) -> bool {
        self.entries.is_empty()
    }
}
