//! Lookup precedence for fields in task envelopes.

/// Extract the metadata map from a task JSON envelope.
pub(super) fn task_metadata(
    task: &serde_json::Value,
) -> serde_json::Map<String, serde_json::Value> {
    task.get("task")
        .and_then(|t| t.get("metadata"))
        .or_else(|| task.get("metadata"))
        .and_then(|m| m.as_object())
        .cloned()
        .unwrap_or_default()
}
pub(super) fn target_agent<'a>(
    task: &'a serde_json::Value,
    metadata: &'a serde_json::Map<String, serde_json::Value>,
) -> Option<&'a str> {
    metadata
        .get("target_agent_name")
        .and_then(|v| v.as_str())
        .or_else(|| task.get("target_agent_name").and_then(|v| v.as_str()))
        .or_else(|| {
            task.get("task")
                .and_then(|t| t.get("target_agent_name"))
                .and_then(|v| v.as_str())
        })
}
