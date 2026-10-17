//! Worker configuration adapters for shared clone locks.
use super::{env_u64, git_clone_base_dir};
use anyhow::Result;
use std::path::{Path, PathBuf};
pub(super) fn workspace_clone_lock_path(workspace_id: &str) -> PathBuf {
    codetether_worktree::repository::clone_lock::workspace_clone_lock_path(
        workspace_id,
        git_clone_base_dir,
    )
}
pub(super) async fn acquire_workspace_clone_lock(lock_path: &Path) -> Result<tokio::fs::File> {
    codetether_worktree::repository::clone_lock::acquire_workspace_clone_lock(lock_path, || {
        env_u64("CODETETHER_WORKER_CLONE_LOCK_STALE_SECS", 1800)
    })
    .await
}
