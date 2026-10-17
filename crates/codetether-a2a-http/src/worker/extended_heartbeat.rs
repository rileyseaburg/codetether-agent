//! Signed request transport for extended task heartbeats.

use anyhow::Result;
use reqwest::{Client, Response};

/// Send the signed extended-heartbeat payload.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker sending the heartbeat.
/// * `task_id` - Task bound to the heartbeat payload.
/// * `payload` - Extended heartbeat JSON body.
/// # Returns
/// The HTTP response, leaving status handling to the caller.
/// # Errors
/// Returns resource serialization, proof configuration, or transport errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::extended_heartbeat::send;
/// let response = send(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", "task", &serde_json::json!({})).await?;
/// # Ok(()) }
/// ```
pub async fn send(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task_id: &str,
    payload: &serde_json::Value,
) -> Result<Response> {
    let request = client
        .post(format!("{server}/v1/worker/tasks/heartbeat-extended"))
        .json(payload);
    let resource = super::proof::mutation_resource(task_id, payload)?;
    let mut request =
        super::proof::apply_configured(request, "heartbeat-extended", worker_id, &resource)?;
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    Ok(request.send().await?)
}
