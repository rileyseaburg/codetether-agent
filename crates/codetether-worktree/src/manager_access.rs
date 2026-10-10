//! Read-only access to the repository used by a worktree manager.

use super::WorktreeManager;
use std::path::Path;

impl WorktreeManager {
    /// Return the Git repository root used for worktree and branch operations.
    ///
    /// # Arguments
    ///
    /// Uses the repository selected when constructing this manager.
    ///
    /// # Returns
    ///
    /// A borrowed path; callers cannot change the manager's repository.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_worktree::WorktreeManager;
    /// let manager = WorktreeManager::for_repo("/srv/project");
    /// assert_eq!(manager.repo_path(), std::path::Path::new("/srv/project"));
    /// ```
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
