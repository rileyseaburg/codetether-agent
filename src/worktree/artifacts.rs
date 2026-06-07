use super::{WorktreeManager, artifact_collect};
use anyhow::Result;
use std::io::ErrorKind;
use std::path::PathBuf;

impl WorktreeManager {
    /// List `target` directories inside CodeTether worktrees.
    pub async fn build_artifact_dirs(&self) -> Result<Vec<PathBuf>> {
        let mut dirs = Vec::new();
        let entries = match tokio::fs::read_dir(&self.base_dir).await {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(dirs),
            Err(error) => return Err(error.into()),
        };
        artifact_collect::push_worktree_targets(&mut dirs, entries).await?;
        artifact_collect::push_existing_children(&mut dirs, &self.base_dir.join(".targets"))
            .await?;
        dirs.sort();
        Ok(dirs)
    }

    /// Remove build artifacts inside worktrees without deleting source checkout state.
    pub async fn cleanup_build_artifacts(&self) -> Result<usize> {
        let dirs = self.build_artifact_dirs().await?;
        for dir in &dirs {
            tokio::fs::remove_dir_all(dir).await?;
        }
        tracing::info!(count = dirs.len(), "Cleaned CodeTether worktree artifacts");
        Ok(dirs.len())
    }
}
