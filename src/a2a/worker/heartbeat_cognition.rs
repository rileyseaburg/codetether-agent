//! Cognition snapshot retrieval for worker heartbeats.

use std::time::Duration;

use reqwest::Client;

use super::{
    CognitionHeartbeatConfig, CognitionLatestSnapshot, CognitionStatusSnapshot, trim_for_heartbeat,
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
    let obj = payload.as_object_mut()?;
    obj.insert(
        "latest_snapshot_at".into(),
        serde_json::Value::String(snapshot.generated_at),
    );
    obj.insert(
        "latest_thought".into(),
        serde_json::Value::String(trim_for_heartbeat(
            &snapshot.summary,
            config.summary_max_chars,
        )),
    );
    if let Some(model) = snapshot
        .metadata
        .get("model")
        .and_then(serde_json::Value::as_str)
    {
        obj.insert(
            "latest_thought_model".into(),
            serde_json::Value::String(model.to_string()),
        );
    }
    if let Some(source) = snapshot
        .metadata
        .get("source")
        .and_then(serde_json::Value::as_str)
    {
        obj.insert(
            "latest_thought_source".into(),
            serde_json::Value::String(source.to_string()),
        );
    }
    Some(payload)
}
