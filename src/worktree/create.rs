//! Public entry points for managed worktree allocation.

use super::{WorktreeInfo, WorktreeManager};
use anyhow::Result;

#[path = "create_impl.rs"]
mod implementation;
#[path = "create_revision.rs"]
mod revision;
#[cfg(test)]
#[path = "create_tests.rs"]
mod tests;

impl WorktreeManager {
    /// Create a new Git worktree for a task.
    pub async fn create(&self, name: &str) -> Result<WorktreeInfo> {
        implementation::create(self, name, None).await
    }

    /// Create a fresh task branch at an explicit commit in managed storage.
    ///
    /// # Arguments
    ///
    /// * `name` — Valid task name; the new branch is `codetether/{name}`.
    /// * `start_point` — Commit ID or revision resolving to a commit in this
    ///   manager's repository. Pass the parent's resolved HEAD commit ID when
    ///   the parent runs in another checkout; `HEAD` here means the manager's.
    ///
    /// # Returns
    ///
    /// Registered worktree information beneath this manager's workspace root
    /// in `.codetether-worktrees/`. Uncommitted parent edits are not copied.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid storage, names, revisions, repository integrity
    /// failures, or Git/filesystem failures. Existing branches are never reused.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// use codetether_agent::worktree::WorktreeManager;
    /// let manager = WorktreeManager::for_repo("/srv/project").without_vscode_auto_open();
    /// let child = manager.create_from("child", "HEAD").await?;
    /// assert_eq!(child.branch, "codetether/child");
    /// # Ok(()) }
    /// ```
    pub async fn create_from(&self, name: &str, start_point: &str) -> Result<WorktreeInfo> {
        implementation::create(self, name, Some(start_point)).await
    }
}
