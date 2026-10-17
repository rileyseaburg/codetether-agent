//! Fetch pending work without dispatching an executor.
use super::summarize_response_body;
use anyhow::Result;

/// Fetch pending task envelopes for an optional named agent.
///
/// # Arguments
/// * `client` - Configured HTTP client.
/// * `server` - Control-plane base URL.
/// * `token` - Optional bearer token.
/// * `agent_name` - Agent filter; empty omits the query parameter.
///
/// # Returns
/// Task envelopes, or an empty vector after an unsuccessful HTTP response.
///
/// # Errors
/// Returns request or JSON-decoding errors.
pub async fn pending_tasks(
    client: &reqwest::Client,
    server: &str,
    token: &Option<String>,
    agent_name: &str,
) -> Result<Vec<serde_json::Value>> {
    let mut url = format!("{}/v1/agent/tasks?status=pending", server);
    if !agent_name.is_empty() {
        url = format!("{}&agent_name={}", url, urlencoding::encode(&agent_name));
    }
    let mut request = client.get(&url);
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
        tracing::warn!(target: "codetether_agent::a2a::worker::pending_tasks",
            %status,
            body = %summarize_response_body(&body),
            "Failed to fetch pending tasks"
        );
        return Ok(Vec::new());
    }
    let data: serde_json::Value = response.json().await?;
    Ok(data
        .as_array()
        .cloned()
        .or_else(|| data["tasks"].as_array().cloned())
        .unwrap_or_default())
}
