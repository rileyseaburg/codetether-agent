//! Metadata extraction helpers for task context.

pub(super) fn tier_from_metadata(
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity: Option<&str>,
) -> Option<String> {
    super::metadata_str(metadata, &["model_tier", "tier"])
        .map(|v| v.to_ascii_lowercase())
        .or_else(|| complexity.map(complexity_tier))
}

fn complexity_tier(value: &str) -> String {
    match value.to_ascii_lowercase().as_str() {
        "quick" => "fast",
        "deep" => "heavy",
        _ => "balanced",
    }
    .to_string()
}

pub(super) fn trimmed_metadata(
    metadata: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    metadata
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
}

pub(super) fn context_workspace_id(
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    super::task_str(task, "workspace_id")
        .or_else(|| super::metadata_lookup(metadata, "workspace_id").and_then(|v| v.as_str()))
        .map(ToString::to_string)
}

pub(super) fn context_selected_model(
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    super::task_str(task, "model_ref")
        .or_else(|| super::metadata_lookup(metadata, "model_ref").and_then(|v| v.as_str()))
        .or_else(|| super::task_str(task, "model"))
        .or_else(|| super::metadata_lookup(metadata, "model").and_then(|v| v.as_str()))
        .map(super::model_ref_to_provider_model)
}
