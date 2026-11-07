//! Install CodeTether's provenance commit-message hook without displacing a
//! repository-owned one.
use anyhow::{Context, Result};
use codetether_provenance::{commit_msg_hook_script, install_commit_msg_hook};
use std::path::Path;

/// Install the provenance `commit-msg` hook unless the repository owns it.
///
/// A `commit-msg` hook that is not CodeTether's script (for example one
/// written by `pre-commit install --hook-type commit-msg`) is left in place
/// and `false` is returned; trailers are then still pre-seeded by
/// `ensure_provenance_trailers`. Otherwise the script is written (or
/// refreshed) and `true` is returned.
///
/// # Errors
/// Returns Git path lookup or hook write errors.
pub(super) async fn install_provenance_hook(directory: &Path) -> Result<bool> {
    let path = codetether_worktree::repository::git::run_git_command_at(
        Some(directory),
        vec![
            "rev-parse".into(),
            "--path-format=absolute".into(),
            "--git-path".into(),
            "hooks/commit-msg".into(),
        ],
    )
    .await
    .context("Failed to resolve commit-msg hook path")?;
    let hook = Path::new(path.trim());
    if let Ok(existing) = tokio::fs::read_to_string(hook).await
        && existing != commit_msg_hook_script()
    {
        return Ok(false);
    }
    install_commit_msg_hook(directory)?;
    Ok(true)
}
