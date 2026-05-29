//! Low-level git clone execution.

use anyhow::Result;

use super::clone_git_cmd::run_git_command_at;
use super::clone_task_data::CloneRepoTask;

pub(super) async fn git_clone_repo(task: &CloneRepoTask<'_>) -> Result<()> {
    super::prepare_clone_target(task.repo_path).await?;
    run_git_command_at(None, vec![
        "-c".into(), format!("credential.helper={}", task.temp_helper_path.display()),
        "-c".into(), "credential.useHttpPath=true".into(),
        "clone".into(), "--single-branch".into(),
        "--branch".into(), task.branch.to_string(),
        task.git_url.to_string(), task.repo_path.display().to_string(),
    ]).await?;
    Ok(())
}
