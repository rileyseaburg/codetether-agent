//! Workspace-rooted [`WorktreeManager`] constructors.

use super::{WorktreeManager, storage};
use std::path::PathBuf;
use tokio::sync::Mutex;

impl WorktreeManager {
    /// Create a manager rooted at `workspace_root`.
    ///
    /// Managed checkouts are stored in `<workspace_root>/.codetether-worktrees`.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::worktree::WorktreeManager;
    /// let manager = WorktreeManager::new("/srv/project");
    /// drop(manager);
    /// ```
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        Self::for_repo(workspace_root)
    }

    /// Retain the legacy signature while ignoring its custom storage path.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::worktree::WorktreeManager;
    /// let manager = WorktreeManager::with_repo("/tmp/ignored", "/srv/project");
    /// drop(manager);
    /// ```
    pub fn with_repo(_base_dir: impl Into<PathBuf>, repo_path: impl Into<PathBuf>) -> Self {
        Self::for_repo(repo_path)
    }

    /// Create a manager at the Git root containing `repo_path`.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::worktree::WorktreeManager;
    /// let manager = WorktreeManager::for_repo("/srv/project/src");
    /// drop(manager);
    /// ```
    pub fn for_repo(repo_path: impl Into<PathBuf>) -> Self {
        let repo_path = repo_path.into();
        let repo_path = crate::provenance::repo_root(&repo_path)
            .ok()
            .flatten()
            .unwrap_or(repo_path);
        Self {
            base_dir: storage::base_dir(&repo_path),
            repo_path,
            worktrees: Mutex::new(Vec::new()),
            integrity_checked: Mutex::new(false),
            auto_open_vscode: true,
        }
    }
}
