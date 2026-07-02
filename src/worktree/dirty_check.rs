//! Dirty-check guard for worktree removal (issue #297 Part B).
//!
//! Before tearing down a worktree with `git worktree remove --force`,
//! check whether it has uncommitted changes. If it does, log a prominent
//! warning so in-flight edits are not silently destroyed.

use super::WorktreeInfo;

impl super::WorktreeManager {
    /// Returns `true` when the worktree directory has uncommitted changes.
    pub(crate) async fn is_worktree_dirty(&self, info: &WorktreeInfo) -> bool {
        let output = tokio::process::Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&info.path)
            .output()
            .await;
        match output {
            Ok(o) if o.status.success() => !o.stdout.is_empty(),
            _ => false,
        }
    }
}
