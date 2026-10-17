//! Credential configuration operation: configure repo git github app.
use super::*;

/// Stores GitHub App identifiers in repository-local Git config.
///
/// Empty values are ignored so callers can pass optional server metadata.
///
/// # Examples
///
/// ```ignore
/// configure_repo_git_github_app(repo_path, Some("1"), Some("2"))?;
/// ```
pub fn configure_repo_git_github_app(
    repo_path: &Path,
    installation_id: Option<&str>,
    app_id: Option<&str>,
) -> Result<()> {
    set_local_config(
        repo_path,
        "codetether.githubInstallationId",
        installation_id,
    )?;
    set_local_config(repo_path, "codetether.githubAppId", app_id)?;
    Ok(())
}
