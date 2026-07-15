//! Cognition snapshot retrieval for worker heartbeats.

use std::time::Duration;

use reqwest::Client;

use super::{
    CognitionHeartbeatConfig, CognitionLatestSnapshot, CognitionStatusSnapshot,
    heartbeat_cognition_merge,
};

pub(super) async fn fetch_cognition_heartbeat_payload(
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
    heartbeat_cognition_merge::merge_snapshot(&mut payload, snapshot, config);
    Some(payload)
}
