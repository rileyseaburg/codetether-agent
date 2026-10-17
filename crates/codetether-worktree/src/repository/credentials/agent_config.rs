//! Workspace metadata driven repository credential configuration.
use super::*;
/// Persist GitHub App metadata selected from an optional agent configuration.
///
/// # Arguments
/// * `repo_path` - Repository whose Git configuration will be updated.
/// * `agent_config` - Optional configuration with a `git_auth.github_app` object.
/// # Returns
/// Unit; configuration failures are logged and do not propagate.
/// # Examples
/// ```rust,no_run
/// use codetether_worktree::repository::credentials::configure_repo_git_github_app_from_agent_config;
/// configure_repo_git_github_app_from_agent_config(std::path::Path::new("repo"), None);
/// ```

pub fn configure_repo_git_github_app_from_agent_config(
    repo_path: &Path,
    agent_config: Option<&serde_json::Value>,
) {
    let github_app = agent_config
        .and_then(|value| value.get("git_auth"))
        .and_then(|value| value.get("github_app"));
    let installation_id = github_app
        .and_then(|value| value.get("installation_id"))
        .and_then(|value| value.as_str());
    let app_id = github_app
        .and_then(|value| value.get("app_id"))
        .and_then(|value| value.as_str());
    if let Err(error) = configure_repo_git_github_app(repo_path, installation_id, app_id) {
        tracing::debug!(target: "codetether_agent::a2a::worker::workspace_sync_git", path = %repo_path.display(), error = %error, "Failed to persist GitHub App repo metadata");
    }
}
