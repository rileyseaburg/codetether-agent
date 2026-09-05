//! Shared allocation and successful-registration lifecycle.

use super::{WorktreeInfo, WorktreeManager};
use anyhow::{Context, Result};

pub(super) async fn create(
    manager: &WorktreeManager,
    name: &str,
    start_point: Option<&str>,
) -> Result<WorktreeInfo> {
    manager.validate_storage()?;
    manager.ensure_repo_integrity_once().await?;
    WorktreeManager::validate_worktree_name(name)?;
    let commit = match start_point {
        Some(point) => Some(super::revision::resolve(manager, point).await?),
        None => None,
    };
    let worktree_path = manager.base_dir.join(name);
    let branch_name = format!("codetether/{name}");
    tokio::fs::create_dir_all(&manager.base_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to create base directory: {}",
                manager.base_dir.display()
            )
        })?;
    let mut output = manager
        .add_worktree(&branch_name, &worktree_path, true, commit.as_deref())
        .await?;
    if !output.status.success() && start_point.is_none() {
        output = manager
            .add_worktree(&branch_name, &worktree_path, false, None)
            .await?;
    }
    if !output.status.success() {
        anyhow::bail!(
            "Failed to create git worktree '{name}': {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let info = WorktreeInfo {
        name: name.to_string(),
        path: worktree_path.clone(),
        branch: branch_name,
        active: true,
    };
    manager.worktrees.lock().await.push(info.clone());
    manager.prepare_validation_dependencies(&worktree_path);
    tracing::info!(worktree = %name, path = %worktree_path.display(), "Created git worktree");
    manager.auto_open_in_vscode(&info).await;
    Ok(info)
}
