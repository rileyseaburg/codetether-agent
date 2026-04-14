use super::RegisteredWorkspaceRecord;
use anyhow::{Context, Result};
use reqwest::Client;

/// Fetch a workspace record from the A2A control plane.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use axum::{Json, Router, routing::get};
/// use codetether_agent::a2a::worker_workspace_record::fetch_workspace_record;
/// use reqwest::Client;
/// use serde_json::json;
/// use tokio::net::TcpListener;
///
/// let listener = TcpListener::bind("127.0.0.1:0").await.expect("listener");
/// let addr = listener.local_addr().expect("addr");
/// tokio::spawn(async move {
///     let app = Router::new().route(
///         "/v1/agent/workspaces/workspace-1",
///         get(|| async {
///             Json(json!({
///                 "id": "workspace-1",
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
///
/// let record = fetch_workspace_record(
///     &Client::new(),
///     &format!("http://{addr}"),
///     &None,
///     "workspace-1",
/// )
/// .await
/// .expect("workspace");
///
/// assert_eq!(record.name, "example");
/// # });
/// ```
pub async fn fetch_workspace_record(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_id: &str,
) -> Result<RegisteredWorkspaceRecord> {
    let mut req = client.get(format!(
        "{}/v1/agent/workspaces/{}",
        server.trim_end_matches('/'),
        workspace_id
    ));
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }
    let response = req
        .send()
        .await
        .with_context(|| format!("Failed to load workspace {workspace_id}"))?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Failed to fetch workspace {workspace_id} ({status}): {body}");
    }
    response
        .json::<RegisteredWorkspaceRecord>()
        .await
        .with_context(|| format!("Failed to decode workspace {workspace_id} response"))
}
