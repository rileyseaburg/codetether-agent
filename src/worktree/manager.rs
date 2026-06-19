use super::WorktreeInfo;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;

/// Worktree manager for creating and managing isolated git worktrees.
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
    /// Create a manager with an explicit worktree storage directory.
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        let repo_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_repo(base_dir, repo_path)
    }

    /// Create a manager with explicit worktree storage and repository paths.
    pub fn with_repo(base_dir: impl Into<PathBuf>, repo_path: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            repo_path: repo_path.into(),
            worktrees: Mutex::new(Vec::new()),
            integrity_checked: Mutex::new(false),
            auto_open_vscode: true,
        }
    }

    /// Create a manager for a repository using `.codetether-worktrees`.
    pub fn for_repo(repo_path: impl Into<PathBuf>) -> Self {
        let repo_path = repo_path.into();
        Self::with_repo(repo_path.join(".codetether-worktrees"), repo_path)
    }

    /// Disable VS Code auto-open for worktrees created by this manager.
    ///
    /// The TUI runs with the terminal in raw mode and the alternate screen
    /// active, so the interactive VS Code prompt would corrupt the display and
    /// can never open an editor over an SSH terminal.
    pub fn without_vscode_auto_open(mut self) -> Self {
        self.auto_open_vscode = false;
        self
    }

    /// Inject a Cargo workspace stub for Cargo workspace isolation.
    pub fn inject_workspace_stub(&self, worktree_path: &Path) -> anyhow::Result<()> {
        crate::worktree_stub::inject(worktree_path, &self.base_dir)
    }
}

impl Default for WorktreeManager {
    fn default() -> Self {
        Self::new("/tmp/codetether-worktrees")
    }
}
