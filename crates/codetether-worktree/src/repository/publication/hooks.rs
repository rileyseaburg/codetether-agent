//! Resolve Git hook directories and enforce publication prerequisites.
use super::*;
use std::path::PathBuf;

async fn hooks_dir(repo_path: &Path) -> Result<PathBuf> {
    let path = git(
        repo_path,
        &["rev-parse", "--path-format=absolute", "--git-path", "hooks"],
    )
    .await?;
    Ok(PathBuf::from(path.trim()))
}

/// Require all declared hooks before worker commits or pushes.
///
/// # Arguments
/// * `repo_path` - Repository whose declared hooks must be installed.
/// # Returns
/// Unit when the repository hook policy is satisfied.
/// # Errors
/// Returns Git path lookup errors or missing/non-executable hook errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::publication::require_installed;
/// require_installed(std::path::Path::new("repo")).await?;
/// # Ok(()) }
/// ```
pub async fn require_installed(repo_path: &Path) -> Result<()> {
    super::super::require_installed(repo_path, || hooks_dir(repo_path)).await
}
