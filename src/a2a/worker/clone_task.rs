//! Clone-repo task execution.

use anyhow::{Context, Result};
use reqwest::Client;

use super::clone_lock::{acquire_workspace_clone_lock, workspace_clone_lock_path};
use super::clone_lock_release::release_workspace_clone_lock;
use super::clone_task_data::CloneRepoTask;
use super::clone_task_result::handle_clone_repo_task_result;
use super::{
    fetch_workspace_record, git_clone_base_dir, metadata_str, resolve_workspace_clone_path,
    task_str, write_git_credential_helper_script,
};

pub(super) async fn handle_clone_repo_task(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Result<String> {
    let workspace_id = metadata_str(metadata, &["workspace_id"])
        .or_else(|| task_str(task, "workspace_id").map(ToString::to_string))
        .ok_or_else(|| anyhow::anyhow!("Clone task is missing workspace_id metadata"))?;
    let workspace = fetch_workspace_record(client, server, token, &workspace_id).await?;
    let git_url = metadata_str(metadata, &["git_url"])
        .or_else(|| workspace.git_url.clone())
        .ok_or_else(|| anyhow::anyhow!("Workspace {} is missing git_url", workspace_id))?;
    let branch = metadata_str(metadata, &["git_branch"])
        .or_else(|| workspace.git_branch.clone())
        .unwrap_or_else(|| "main".to_string());
    let repo_path = resolve_workspace_clone_path(&workspace_id, &workspace.path);
    if let Some(parent) = repo_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("Failed to create repo parent {}", parent.display()))?;
    }
    let temp_helper_path =
        git_clone_base_dir().join(format!(".{}-git-credential-helper", workspace_id));
    write_git_credential_helper_script(&temp_helper_path, &workspace_id)?;
    let clone_lock =
        acquire_workspace_clone_lock(&workspace_clone_lock_path(&workspace_id)).await?;
    let clone_result = handle_clone_repo_task_result(CloneRepoTask {
        client,
        server,
        token,
        worker_id,
        workspace: &workspace,
        repo_path: &repo_path,
        git_url: &git_url,
        branch: &branch,
        temp_helper_path: &temp_helper_path,
        metadata,
    })
    .await;
    let _ = tokio::fs::remove_file(&temp_helper_path).await;
    release_workspace_clone_lock(clone_lock).await;
    clone_result
}
