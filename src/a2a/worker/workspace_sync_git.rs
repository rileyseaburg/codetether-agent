//! Git-auth helpers used during worker workspace sync.

use std::path::Path;

use crate::a2a::git_credentials::{configure_repo_git_auth, configure_repo_git_github_app};

pub(super) fn maybe_configure_repo_git_auth_from_entry(entry: &serde_json::Value) {
    let Some(workspace_id) = entry.get("id").and_then(|value| value.as_str()) else {
        return;
    };
    let Some(path) = entry.get("path").and_then(|value| value.as_str()) else {
        return;
    };
    let repo_path = Path::new(path);
    let has_git_remote = entry
        .get("git_url")
        .and_then(|value| value.as_str())
        .is_some_and(|value| !value.trim().is_empty());
    if !has_git_remote || !repo_path.join(".git").exists() {
        return;
    }
    if let Err(error) = configure_repo_git_auth(repo_path, workspace_id) {
        tracing::debug!(workspace_id, path, error = %error, "Workspace sync could not install Git credential helper");
    }
    configure_repo_git_github_app_from_agent_config(repo_path, entry.get("agent_config"));
}

pub(super) fn configure_repo_git_github_app_from_agent_config(
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
        tracing::debug!(path = %repo_path.display(), error = %error, "Failed to persist GitHub App repo metadata");
    }
}
