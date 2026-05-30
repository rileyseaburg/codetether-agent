//! Clone-repo task: execute result (git operations and registration).

use anyhow::Result;

use super::clone_enqueue::enqueue_post_clone_task;
use super::clone_git::{refresh_existing_clone, run_git_command_at};
use super::clone_repo_exec;
use super::clone_task_data::CloneRepoTask;
use super::{
    configure_repo_git_auth, configure_repo_git_github_app_from_agent_config,
    install_commit_msg_hook, prepare_clone_target, register_cloned_workspace,
    worker_should_enqueue_post_clone_task,
};

pub(super) async fn handle_clone_repo_task_result(task: CloneRepoTask<'_>) -> Result<String> {
    let agent_config = serde_json::Value::Object(task.workspace.agent_config.clone());
    if task.repo_path.join(".git").exists() {
        configure_repo_git_auth(task.repo_path, &task.workspace.id)?;
        configure_repo_git_github_app_from_agent_config(task.repo_path, Some(&agent_config));
        refresh_existing_clone(task.repo_path, task.branch).await?;
    } else {
        clone_repo_exec::git_clone_repo(&task).await?;
        configure_repo_git_auth(task.repo_path, &task.workspace.id)?;
        configure_repo_git_github_app_from_agent_config(task.repo_path, Some(&agent_config));
    }
    install_commit_msg_hook(task.repo_path)?;
    register_cloned_workspace(
        task.client,
        task.server,
        task.token,
        task.worker_id,
        task.workspace,
        task.repo_path,
    )
    .await?;
    if worker_should_enqueue_post_clone_task(task.metadata) {
        enqueue_post_clone_task(
            task.client,
            task.server,
            task.token,
            task.worker_id,
            &task.workspace.id,
            task.metadata,
        )
        .await?;
    }
    Ok(format!(
        "Repository ready at {} (branch: {})",
        task.repo_path.display(),
        task.branch
    ))
}
