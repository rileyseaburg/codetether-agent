//! Git operations used by clone-repo tasks.

use std::path::Path;

use anyhow::Result;

pub(super) use super::clone_git_cmd::run_git_command_at;

pub(super) async fn refresh_existing_clone(repo_path: &Path, branch: &str) -> Result<()> {
    run_git_command_at(
        Some(repo_path),
        vec!["fetch".into(), "origin".into(), branch.into()],
    )
    .await?;
    stash_dirty_clone_state(repo_path).await?;
    let remote_ref = format!("origin/{branch}");
    run_git_command_at(
        Some(repo_path),
        vec![
            "checkout".into(),
            "-B".into(),
            branch.into(),
            remote_ref.clone(),
        ],
    )
    .await?;
    run_git_command_at(
        Some(repo_path),
        vec!["reset".into(), "--hard".into(), remote_ref],
    )
    .await?;
    run_git_command_at(Some(repo_path), vec!["clean".into(), "-fd".into()]).await?;
    Ok(())
}

async fn stash_dirty_clone_state(repo_path: &Path) -> Result<()> {
    if run_git_command_at(Some(repo_path), vec!["status".into(), "--porcelain".into()])
        .await?
        .trim()
        .is_empty()
    {
        return Ok(());
    }
    let message = format!(
        "codetether auto-save before workspace refresh {}",
        chrono::Utc::now().to_rfc3339()
    );
    run_git_command_at(
        Some(repo_path),
        vec![
            "stash".into(),
            "push".into(),
            "--include-untracked".into(),
            "-m".into(),
            message,
        ],
    )
    .await
    .map(|_| ())
}
