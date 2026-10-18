//! Send selected bus events to the configured worker endpoint.
use anyhow::Result;
use codetether_bus_core::BusEnvelope;
use reqwest::Client;

/// Forward bus events to the A2A server for observability
/// # Arguments
/// * `client` - Caller-owned HTTP client.
/// * `server` - Base server URL.
/// * `token` - Bearer token supplied by the caller.
/// * `worker_id` - Registered worker identifier.
/// * `envelope` - Bus event to filter and forward.
/// # Returns
/// Unit after forwarding, or immediately for an unsupported event.
/// # Errors
/// Propagates HTTP transport errors; non-success statuses are logged.
/// # Examples
/// ```no_run
/// # async fn example(event: &codetether_bus_core::BusEnvelope) -> anyhow::Result<()> {
/// codetether_worker_events::forward_bus_event(&reqwest::Client::new(), "http://localhost", "token", "worker", event).await
/// # }
/// ```
pub async fn forward_bus_event(
    client: &Client,
    server: &str,
    token: &str,
    worker_id: &str,
    envelope: &BusEnvelope,
) -> Result<()> {
    let Some(payload) = crate::payload::payload(worker_id, envelope) else {
        return Ok(());
    };
    let url = format!("{}/v1/agent/workers/{}/events", server, worker_id);
    let res = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        tracing::debug!("Failed to forward bus event: {}", res.status());
    }

    Ok(())
}
