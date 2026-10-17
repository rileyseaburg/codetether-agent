//! Connect shared workspace synchronization to repository credential setup.
use super::workspace_sync_git::maybe_configure_repo_git_auth_from_entry;
use anyhow::Result;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
pub(super) async fn sync_workspaces_from_server(
    client: &Client,
    server: &str,
    token: &Option<String>,
    shared_codebases: &Arc<Mutex<Vec<String>>>,
) -> Result<()> {
    codetether_a2a_http::workspace::sync_workspaces_from_server(
        client,
        server,
        token,
        shared_codebases,
        maybe_configure_repo_git_auth_from_entry,
    )
    .await
}
