use super::{WorktreeInfo, WorktreeManager};
use anyhow::{Context, Result, anyhow};

impl WorktreeManager {
    /// Create a new Git worktree for a task.
    pub async fn create(&self, name: &str) -> Result<WorktreeInfo> {
        self.ensure_repo_integrity_once().await?;
        Self::validate_worktree_name(name)?;
        let worktree_path = self.base_dir.join(name);
        let branch_name = format!("codetether/{name}");
        tokio::fs::create_dir_all(&self.base_dir)
            .await
            .with_context(|| {
                format!(
                    "Failed to create base directory: {}",
                    self.base_dir.display()
                )
            })?;
        let first = self
            .add_worktree(&branch_name, &worktree_path, true)
            .await?;
        if !first.status.success() {
            let second = self
                .add_worktree(&branch_name, &worktree_path, false)
                .await?;
            if !second.status.success() {
                return Err(anyhow!(
                    "Failed to create git worktree '{}': {}",
                    name,
                    String::from_utf8_lossy(&second.stderr)
                ));
            }
        }
        let info = WorktreeInfo {
            name: name.to_string(),
            path: worktree_path.clone(),
            branch: branch_name,
            active: true,
        };
        self.worktrees.lock().await.push(info.clone());
        self.prepare_validation_dependencies(&worktree_path);
        tracing::info!(worktree = %name, path = %worktree_path.display(), "Created git worktree");
        self.auto_open_in_vscode(&info).await;
        Ok(info)
    }
}
