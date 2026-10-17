//! Adapt worker clone task inputs to shared repository cloning.
use super::clone_task_data::CloneRepoTask;
use anyhow::Result;
pub(super) async fn git_clone_repo(task: &CloneRepoTask<'_>) -> Result<()> {
    codetether_worktree::repository::git::git_clone_repo(
        task.repo_path,
        task.temp_helper_path,
        task.branch,
        task.git_url,
    )
    .await
}
