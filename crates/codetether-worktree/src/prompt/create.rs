use super::WorktreeState;
use crate::WorktreeManager;
use std::path::{Path, PathBuf};
/// Create an isolated prompt checkout and update its host directory.
/// # Arguments
/// `cwd` is the repository root; `current_branch` reads the original branch;
/// `set_directory` records the successful checkout path in native session state.
/// # Returns
/// Owned cleanup state on success, or None after logging a creation failure.
/// # Examples
/// ```no_run
/// # async fn create() {
/// let _state = codetether_worktree::prompt::create_worktree(
///     std::path::Path::new("."), |_| None, |_| {},
/// ).await;
/// # }
/// ```
pub async fn create_worktree(
    cwd: &Path,
    current_branch: impl FnOnce(&Path) -> Option<String> + Send,
    set_directory: impl FnOnce(PathBuf) + Send,
) -> Option<WorktreeState> {
    let repo_dir = cwd.to_path_buf();
    let name = format!("tui_{}", uuid::Uuid::new_v4().simple());
    let mgr = WorktreeManager::for_repo(&repo_dir).without_vscode_auto_open();
    let base_branch = current_branch(cwd);
    match mgr.create(&name).await {
        Ok(wt) => {
            let _ = mgr.inject_workspace_stub(&wt.path);
            tracing::info!(
                worktree = %name,
                path = %wt.path.display(),
                "Created TUI worktree for prompt isolation"
            );
            set_directory(wt.path.clone());
            Some((mgr, wt, base_branch))
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to create worktree, running in main directory");
            None
        }
    }
}
