//! Whether a mux window gets its own managed worktree.

/// Workspace isolation policy for a mux window.
///
/// `Worktree` is the default and allocates a managed Git worktree under
/// `.codetether-worktrees/`. `Shared` reuses the requested directory as-is,
/// which removes `git worktree add` from the startup path.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Isolation {
    /// Allocate a managed Git worktree for the window.
    #[default]
    Worktree,
    /// Use the requested directory directly with no worktree allocation.
    Shared,
}

impl Isolation {
    /// Build a policy from a `--no-worktree` style flag.
    pub(crate) fn from_no_worktree(no_worktree: bool) -> Self {
        if no_worktree {
            Self::Shared
        } else {
            Self::Worktree
        }
    }

    /// Whether the requested directory is used without worktree allocation.
    pub(crate) fn is_shared(self) -> bool {
        matches!(self, Self::Shared)
    }
}
