use super::{WorktreeInfo, WorktreeManager};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

impl WorktreeManager {
    /// Write a multi-root `.code-workspace` file covering the main repo and
    /// every supplied worktree.
    ///
    /// Each worktree becomes a named folder and
    /// `git.autoRepositoryDetection: "subFolders"` is enabled so VS Code's
    /// Source Control view picks up each linked worktree.
    ///
    /// # Arguments
    ///
    /// * `worktrees` — Worktrees to include as folders.
    /// * `dest` — Destination path for the `.code-workspace` file.
    ///
    /// # Returns
    ///
    /// The path the workspace file was written to.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the file cannot be written.
    pub async fn write_code_workspace(
        &self,
        worktrees: &[WorktreeInfo],
        dest: impl AsRef<Path>,
    ) -> Result<PathBuf> {
        let dest = dest.as_ref().to_path_buf();
        let mut folders = vec![serde_json::json!({
            "name": "main",
            "path": self.repo_path.to_string_lossy(),
        })];
        for wt in worktrees {
            folders.push(serde_json::json!({
                "name": wt.name,
                "path": wt.path.to_string_lossy(),
            }));
        }
        let doc = serde_json::json!({
            "folders": folders,
            "settings": { "git.autoRepositoryDetection": "subFolders" },
        });
        let body = serde_json::to_string_pretty(&doc)?;
        tokio::fs::write(&dest, body)
            .await
            .with_context(|| format!("Failed to write workspace file: {}", dest.display()))?;
        tracing::info!(path = %dest.display(), folders = worktrees.len() + 1, "Wrote VS Code workspace");
        Ok(dest)
    }
}
