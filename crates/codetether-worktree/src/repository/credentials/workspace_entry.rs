//! Workspace metadata driven repository credential configuration.
use super::*;
/// Configure Git credentials for a local workspace catalog entry when applicable.
///
/// # Arguments
/// * `entry` - Workspace metadata containing an ID, path, remote and agent config.
/// # Returns
/// Unit; incomplete metadata and non-repositories are skipped. Configuration
/// failures are logged, and GitHub App metadata is still attempted.
/// # Examples
/// ```rust
/// use codetether_worktree::repository::credentials::maybe_configure_repo_git_auth_from_entry;
/// maybe_configure_repo_git_auth_from_entry(&serde_json::json!({}));
/// ```

pub fn maybe_configure_repo_git_auth_from_entry(entry: &serde_json::Value) {
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
        tracing::debug!(target: "codetether_agent::a2a::worker::workspace_sync_git", workspace_id, path, error = %error, "Workspace sync could not install Git credential helper");
    }
    configure_repo_git_github_app_from_agent_config(repo_path, entry.get("agent_config"));
}
