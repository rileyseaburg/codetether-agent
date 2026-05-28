//! Task context builder helpers.

pub(super) use super::task_context_struct::TaskContext;

pub(super) fn build_task_context(task: &serde_json::Value, title: &str) -> TaskContext {
    let metadata = super::task_metadata(task);
    let complexity_hint = super::metadata_str(&metadata, &["complexity"]);
    let model_tier = tier_from_metadata(&metadata, complexity_hint.as_deref());
    let workspace_id = workspace_id(task, &metadata);
    let resume_session_id = trimmed_metadata(&metadata, "resume_session_id");
    TaskContext {
        worker_personality: super::metadata_str(
            &metadata,
            &["worker_personality", "personality", "agent_personality"],
        ),
        target_agent_name: super::metadata_str(&metadata, &["target_agent_name", "agent_name"]),
        selected_model: selected_model(task, &metadata),
        raw_agent: super::task_str(task, "agent_type")
            .or_else(|| super::task_str(task, "agent"))
            .unwrap_or("build")
            .to_string(),
        prompt: super::task_str(task, "prompt")
            .or_else(|| super::task_str(task, "description"))
            .unwrap_or(title)
            .to_string(),
        is_virtual_task: workspace_id
            .as_deref()
            .is_some_and(|value| value == "global" || value.is_empty()),
        metadata,
        resume_session_id,
        complexity_hint,
        model_tier,
        workspace_id,
    }
}
fn tier_from_metadata(
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity: Option<&str>,
) -> Option<String> {
    super::metadata_str(metadata, &["model_tier", "tier"])
        .map(|value| value.to_ascii_lowercase())
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
fn trimmed_metadata(
    metadata: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    metadata
        .get(key)
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}
fn workspace_id(
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    super::task_str(task, "workspace_id")
        .or_else(|| {
            super::metadata_lookup(metadata, "workspace_id").and_then(|value| value.as_str())
        })
        .map(ToString::to_string)
}
fn selected_model(
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    super::task_str(task, "model_ref")
        .or_else(|| super::metadata_lookup(metadata, "model_ref").and_then(|value| value.as_str()))
        .or_else(|| super::task_str(task, "model"))
        .or_else(|| super::metadata_lookup(metadata, "model").and_then(|value| value.as_str()))
        .map(super::model_ref_to_provider_model)
}
