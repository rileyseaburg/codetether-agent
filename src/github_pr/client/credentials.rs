//! Acquire repository credentials from the A2A server.
use super::*;

pub(super) async fn github_client(context: &RepoContext) -> Result<Client> {
    let server = std::env::var("CODETETHER_SERVER")
        .map_err(|_| anyhow!("CODETETHER_SERVER is required for codetether pr create"))?;
    let token = std::env::var("CODETETHER_TOKEN").ok();
    let worker_id = std::env::var("CODETETHER_WORKER_ID").ok();
    let path = format!("{}/{}.git", context.owner, context.repo);
    tracing::debug!(
        repo_root = %context.repo_root.display(),
        workspace_id = %context.workspace_id,
        github_installation_id = context.github_installation_id.as_deref().unwrap_or(""),
        github_app_id = context.github_app_id.as_deref().unwrap_or(""),
        "Requesting GitHub credentials for PR helper"
    );
    let credentials = request_git_credentials(
        &server,
        &token,
        worker_id.as_deref(),
        &context.workspace_id,
        "get",
        &GitCredentialQuery {
            protocol: Some("https".to_string()),
            host: Some("github.com".to_string()),
            path: Some(path),
        },
    )
    .await?
    .ok_or_else(|| {
        anyhow!(
            "No GitHub credentials available for workspace {}",
            context.workspace_id
        )
    })?;

    codetether_github_pr::http::authenticated_client(&credentials.password)
}
