//! Retrieve cognition status and optional summaries for worker heartbeats.
//!
//! [fetch_cognition_heartbeat_payload] requests status first, then optionally
//! fetches the latest thought. Summary merging and truncation are kept local.
use codetether_a2a_worker_core::{
    CognitionHeartbeatConfig, CognitionLatestSnapshot, CognitionStatusSnapshot,
};
use reqwest::Client;
use std::time::Duration;
mod merge;
mod summary;
/// Build optional cognition heartbeat metadata from the configured service.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `config` - Source URL, timeout and thought-summary settings.
/// # Returns
/// Cognition JSON when required requests and decoding succeed, otherwise `None`.
/// A non-success latest-snapshot response retains the status-only payload.
/// # Examples
/// ```rust,no_run
/// # async fn example(config: &codetether_a2a_worker_core::CognitionHeartbeatConfig) {
/// use codetether_a2a_http::worker::cognition::fetch_cognition_heartbeat_payload;
/// let payload = fetch_cognition_heartbeat_payload(&reqwest::Client::new(), config).await;
/// # }
/// ```

pub async fn fetch_cognition_heartbeat_payload(
    client: &Client,
    config: &CognitionHeartbeatConfig,
) -> Option<serde_json::Value> {
    let status = tokio::time::timeout(
        Duration::from_millis(config.request_timeout_ms),
        client
            .get(format!("{}/v1/cognition/status", config.source_base_url))
            .send(),
    )
    .await
    .ok()?
    .ok()?;
    if !status.status().is_success() {
        return None;
    }
    let status: CognitionStatusSnapshot = status.json().await.ok()?;
    let mut payload = serde_json::json!({ "running": status.running, "last_tick_at": status.last_tick_at, "active_persona_count": status.active_persona_count, "events_buffered": status.events_buffered, "snapshots_buffered": status.snapshots_buffered, "loop_interval_ms": status.loop_interval_ms });
    if !config.include_thought_summary {
        return Some(payload);
    }
    let snapshot = tokio::time::timeout(
        Duration::from_millis(config.request_timeout_ms),
        client
            .get(format!(
                "{}/v1/cognition/snapshots/latest",
                config.source_base_url
            ))
            .send(),
    )
    .await
    .ok()
    .and_then(Result::ok)?;
    if !snapshot.status().is_success() {
        return Some(payload);
    }
    let snapshot: CognitionLatestSnapshot = snapshot.json().await.ok()?;
    merge::merge_snapshot(&mut payload, snapshot, config);
    Some(payload)
}
