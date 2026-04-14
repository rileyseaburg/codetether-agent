use super::resolve_workspace_clone_path;
use crate::a2a::worker_workspace_record::fetch_workspace_record;
use anyhow::Result;
use reqwest::Client;
use std::path::PathBuf;

/// Resolve the local clone directory for a claimed workspace.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use axum::{Json, Router, routing::get};
/// use codetether_agent::a2a::worker_workspace_context::resolve_task_workspace_dir;
/// use reqwest::Client;
/// use serde_json::json;
/// use tokio::net::TcpListener;
///
/// let listener = TcpListener::bind("127.0.0.1:0").await.expect("listener");
/// let addr = listener.local_addr().expect("addr");
/// tokio::spawn(async move {
///     let app = Router::new().route(
///         "/v1/agent/workspaces/workspace-123",
///         get(|| async {
///             Json(json!({
///                 "id": "workspace-123",
///                 "name": "example",
///                 "path": "/tmp/example",
///                 "description": "Example workspace",
///                 "agent_config": {},
///                 "git_url": serde_json::Value::Null,
///                 "git_branch": "main"
///             }))
///         }),
///     );
///     axum::serve(listener, app).await.expect("serve");
/// });
/// let client = Client::new();
/// let path = resolve_task_workspace_dir(
///     &client,
///     &format!("http://{addr}"),
///     &None,
///     Some("workspace-123"),
/// )
/// .await
/// .expect("path");
///
/// assert_eq!(path.unwrap().display().to_string(), "/tmp/example");
/// # });
/// ```
pub async fn resolve_task_workspace_dir(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_id: Option<&str>,
) -> Result<Option<PathBuf>> {
    let Some(workspace_id) = workspace_id.filter(|id| !id.is_empty()) else {
        return Ok(None);
    };
    let workspace = fetch_workspace_record(client, server, token, workspace_id).await?;
    Ok(Some(resolve_workspace_clone_path(
        workspace_id,
        &workspace.path,
    )))
}
