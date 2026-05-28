//! Git operations used by clone-repo tasks.

use std::path::Path;

use anyhow::{Context, Result};
use tokio::process::Command;

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

pub(super) async fn run_git_command_at(
    current_dir: Option<&Path>,
    args: Vec<String>,
) -> Result<String> {
    let mut command = Command::new("git");
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }
    let output = command
        .args(args.iter().map(String::as_str))
        .output()
        .await
        .context("Failed to execute git command")?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }
    Err(anyhow::anyhow!(
        "Git command failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
