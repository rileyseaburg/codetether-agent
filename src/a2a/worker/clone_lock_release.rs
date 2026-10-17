//! Worker directory adapter for shared clone lock cleanup.
use super::git_clone_base_dir;
pub(super) async fn release_workspace_clone_lock(lock: tokio::fs::File) {
    codetether_worktree::repository::clone_lock::release_workspace_clone_lock(
        lock,
        git_clone_base_dir,
    )
    .await
}
