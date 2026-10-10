//! Dirty-check guard for worktree removal (issue #297 Part B).
//!
//! Before tearing down a worktree with `git worktree remove --force`,
//! check whether it has uncommitted changes. If it does — or if the check
//! itself fails — treat the worktree as dirty so in-flight edits are never
//! silently destroyed.

use super::WorktreeInfo;

impl super::WorktreeManager {
    /// Returns `true` when the worktree has uncommitted changes, or when the
    /// dirty check could not be completed (fail-safe: never assume clean).
    pub(crate) async fn is_worktree_dirty(&self, info: &WorktreeInfo) -> bool {
        match tokio::process::Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&info.path)
            .output()
            .await
        {
            Ok(o) if o.status.success() => !o.stdout.is_empty(),
            Ok(o) => {
                tracing::error!(
                    worktree = %info.name,
                    stderr = %String::from_utf8_lossy(&o.stderr),
                    "git status dirty check failed; treating worktree as dirty"
                );
                true
            }
            Err(e) => {
                tracing::error!(
                    worktree = %info.name, error = %e,
                    "failed to run git status dirty check; treating worktree as dirty"
                );
                true
            }
        }
    }
}
