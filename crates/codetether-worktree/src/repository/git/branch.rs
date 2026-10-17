//! Repository Git branch operation.
use super::*;

/// Check out a requested task branch when both repository and branch are supplied.
///
/// # Arguments
/// * `repo_path` - Optional repository directory.
/// * `branch` - Optional task branch name.
/// # Returns
/// Unit; absent metadata, blank branches and non-repositories are skipped.
/// # Errors
/// Returns an error if Git cannot inspect or check out the requested branch.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::git::ensure_metadata_checked_out;
/// ensure_metadata_checked_out(None, None).await?;
/// # Ok(()) }
/// ```
pub async fn ensure_metadata_checked_out(
    repo_path: Option<&Path>,
    branch: Option<String>,
) -> Result<()> {
    if let (Some(repo_path), Some(branch)) = (repo_path, branch) {
        ensure_checked_out(repo_path, &branch).await?;
    }
    Ok(())
}
