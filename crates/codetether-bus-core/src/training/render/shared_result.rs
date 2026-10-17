//! SharedResult training record projection.
use super::*;
pub(super) fn build(
    key: &String,
    value: &serde_json::Value,
    tags: &Vec<String>,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Shared result `{key}` [{}]: {}",
            tags.join(", "),
            serde_json::to_string(value).unwrap_or_default()
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
