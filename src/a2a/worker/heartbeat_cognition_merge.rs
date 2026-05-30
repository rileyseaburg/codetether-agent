//! Cognition snapshot merge into heartbeat payload.

use super::{CognitionHeartbeatConfig, CognitionLatestSnapshot, trim_for_heartbeat};

pub(super) fn merge_snapshot(
    payload: &mut serde_json::Value,
    snapshot: CognitionLatestSnapshot,
    config: &CognitionHeartbeatConfig,
) {
    let obj = match payload.as_object_mut() {
        Some(o) => o,
        None => return,
    };
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
}
