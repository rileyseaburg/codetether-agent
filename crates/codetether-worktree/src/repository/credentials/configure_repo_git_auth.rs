//! Credential configuration operation: configure repo git auth.
use super::*;

/// Configures a repository to use the CodeTether Git credential helper.
///
/// The helper path is returned so callers can inspect or clean it up later.
///
/// # Examples
///
/// ```ignore
/// let helper = configure_repo_git_auth(repo_path, "ws-1")?;
/// ```
pub fn configure_repo_git_auth(repo_path: &Path, workspace_id: &str) -> Result<PathBuf> {
    exclude_legacy_worktree_helper(repo_path)?;
    let helper_path = repo_git_path(repo_path, "codetether-credential-helper")?;
    write_git_credential_helper_script(&helper_path, workspace_id)?;
    let helper_path_str = helper_path
        .to_str()
        .ok_or_else(|| anyhow!("Helper path is not valid UTF-8"))?;
    run_git_config_command(
        repo_path,
        &[
            "config",
            "--local",
            "--replace-all",
            "credential.helper",
            helper_path_str,
        ],
    )?;
    run_git_config_command(
        repo_path,
        &["config", "--local", "credential.useHttpPath", "true"],
    )?;
    run_git_config_command(
        repo_path,
        &["config", "--local", "codetether.workspaceId", workspace_id],
    )?;
    Ok(helper_path)
}
