use serde::Serialize;

/// Safety decision for one Git-registered worktree.
///
/// `Ready` and `Prunable` are the only preview states cleanup may remove.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::worktree::maintenance::WorktreeCleanupState;
///
/// assert_eq!(WorktreeCleanupState::Ready.as_str(), "ready");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorktreeCleanupState {
    /// The repository's primary checkout, which is always protected.
    Primary,
    /// The checkout from which cleanup is running, which is protected.
    Current,
    /// A clean checkout whose commit is contained in the cleanup base.
    Ready,
    /// A checkout with uncommitted or untracked files.
    Dirty,
    /// A clean checkout with commits not contained in the cleanup base.
    Unmerged,
    /// A checkout explicitly locked through Git.
    Locked,
    /// Stale Git metadata for a checkout whose directory is missing.
    Prunable,
    /// A checkout whose state could not be established safely.
    InspectionFailed,
    /// A checkout or stale registration removed during apply mode.
    Removed,
    /// A checkout Git refused to remove during apply mode.
    RemovalFailed,
}

impl WorktreeCleanupState {
    /// Return the stable text label used by human-readable reports.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Current => "current",
            Self::Ready => "ready",
            Self::Dirty => "dirty",
            Self::Unmerged => "unmerged",
            Self::Locked => "locked",
            Self::Prunable => "prunable",
            Self::InspectionFailed => "inspection_failed",
            Self::Removed => "removed",
            Self::RemovalFailed => "removal_failed",
        }
    }
}
