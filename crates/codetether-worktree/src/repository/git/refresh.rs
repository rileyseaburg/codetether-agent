//! Repository Git refresh operation.
use super::*;

/// Refresh a clone to the fetched branch after preserving dirty state in a stash.
///
/// # Arguments
/// * `repo_path` - Existing clone directory.
/// * `branch` - Branch fetched from origin and checked out locally.
/// # Returns
/// Unit after fetch, optional stash, checkout, reset, and clean succeed.
/// # Errors
/// Returns the first Git command error.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::git::refresh_existing_clone;
/// refresh_existing_clone(std::path::Path::new("repo"), "main").await?;
/// # Ok(()) }
/// ```
pub async fn refresh_existing_clone(repo_path: &Path, branch: &str) -> Result<()> {
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
