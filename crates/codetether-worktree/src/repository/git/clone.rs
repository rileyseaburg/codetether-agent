//! Clone with a temporary per-command credential helper.
use super::*;
/// Clone a single branch after validating the destination.
///
/// # Arguments
/// * `repo_path` - Destination to validate before cloning.
/// * `temp_helper_path` - Credential helper used only by this command.
/// * `branch` - Remote branch to clone.
/// * `git_url` - Remote repository URL.
/// # Returns
/// Unit after a successful single-branch clone.
/// # Errors
/// Returns destination validation or Git process errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use std::path::Path;
/// use codetether_worktree::repository::git::git_clone_repo;
/// git_clone_repo(Path::new("repo"), Path::new("helper"), "main", "https://example.com/repo.git").await?;
/// # Ok(()) }
/// ```

pub async fn git_clone_repo(
    repo_path: &Path,
    temp_helper_path: &Path,
    branch: &str,
    git_url: &str,
) -> Result<()> {
    super::super::prepare_clone_target(repo_path).await?;
    run_git_command_at(
        None,
        vec![
            "-c".into(),
            format!("credential.helper={}", temp_helper_path.display()),
            "-c".into(),
            "credential.useHttpPath=true".into(),
            "clone".into(),
            "--single-branch".into(),
            "--branch".into(),
            branch.to_string(),
            git_url.to_string(),
            repo_path.display().to_string(),
        ],
    )
    .await?;
    Ok(())
}
