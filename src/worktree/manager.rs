use super::WorktreeInfo;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;

#[path = "manager_constructors.rs"]
mod constructors;
#[path = "manager_default.rs"]
mod default_impl;
#[path = "storage.rs"]
mod storage;

/// Worktree manager for creating and managing isolated Git worktrees.
///
/// # Examples
///
/// ```
/// use codetether_agent::worktree::WorktreeManager;
/// let manager = WorktreeManager::for_repo("/srv/project");
/// drop(manager);
/// ```
#[derive(Debug)]
pub struct WorktreeManager {
    pub(crate) base_dir: PathBuf,
    pub(crate) repo_path: PathBuf,
    pub(crate) worktrees: Mutex<Vec<WorktreeInfo>>,
    pub(crate) integrity_checked: Mutex<bool>,
    /// Whether to attempt opening freshly created worktrees in VS Code.
    /// TUI callers disable this because the blocking prompt corrupts the
    /// raw-mode terminal and cannot open an editor over SSH.
    pub(crate) auto_open_vscode: bool,
}

impl WorktreeManager {
    /// Disable VS Code auto-open for worktrees created by this manager.
    ///
    /// The TUI runs with the terminal in raw mode and the alternate screen
    /// active, so the interactive VS Code prompt would corrupt the display and
    /// can never open an editor over an SSH terminal.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::worktree::WorktreeManager;
    /// let manager = WorktreeManager::for_repo("/srv/project").without_vscode_auto_open();
    /// drop(manager);
    /// ```
    pub fn without_vscode_auto_open(mut self) -> Self {
        self.auto_open_vscode = false;
        self
    }

    /// Inject a Cargo workspace stub for Cargo workspace isolation.
    ///
    /// # Errors
    ///
    /// Returns an error when the worktree files or Git index cannot be updated.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::worktree::WorktreeManager;
    /// let manager = WorktreeManager::for_repo("/srv/project");
    /// manager.inject_workspace_stub(std::path::Path::new("/srv/project/.codetether-worktrees/task"))?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn inject_workspace_stub(&self, worktree_path: &Path) -> anyhow::Result<()> {
        crate::worktree_stub::inject(worktree_path, &self.base_dir)
    }
}

#[cfg(test)]
#[path = "manager_tests.rs"]
mod tests;
