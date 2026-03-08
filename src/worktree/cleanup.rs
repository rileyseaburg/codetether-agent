//! Cleanup operations for worktrees
use crate::worktree::helpers::default_base_dir_for_repo;
use crate::worktree::types::WorktreeInfo;
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::{process::Command, fs};

pub struct CleanupManager;

impl CleanupManager {
    pub async fn remove_worktree(repo_path: &Path, worktree_path: &Path) -> Result<()> {
        let output = Command::new("git")
            .args(["worktree", "remove", "--force"])
            .arg(worktree_path)
            .current_dir(repo_path)
            .output()
            .await;

        match output {
            Ok(o) if o.status.success() => {
                tracing::info!("Removed git worktree");
            }
            Ok(_o) => {
                tracing::warn!("git worktree remove failed, using fallback");
                let _ = fs::remove_dir_all(worktree_path).await;
            }
            Err(e) => {
                tracing::warn!("Failed to remove worktree: {}", e);
                let _ = fs::remove_dir_all(worktree_path).await;
            }
        }
        Ok(())
    }

    pub async fn cleanup_all(repo_path: &Path, worktrees: &[WorktreeInfo]) -> Result<usize> {
        let count = worktrees.len();
        for info in worktrees {
            let _ = Command::new("git")
                .args(["worktree", "remove", "--force"])
                .arg(&info.path)
                .current_dir(repo_path)
                .output()
                .await;
            let _ = fs::remove_dir_all(&info.path).await;
        }
        tracing::info!(count, "Cleaned up all worktrees");
        Ok(count)
    }

    pub async fn remove_worktree_by_path(repo_path: &Path, name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Worktree name cannot be empty"));
        }
        let path = default_base_dir_for_repo(repo_path).join(name);
        CleanupManager::remove_worktree(repo_path, &path).await
    }
}
