//! ToolRequest training record projection.
use super::*;
pub(super) fn build(
    request_id: &String,
    tool_name: &String,
    arguments: &serde_json::Value,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "assistant".into(),
        content: None,
        tool_calls: Some(vec![TrainingToolCall {
            id: request_id.clone(),
            call_type: "function".into(),
            function: TrainingFunction {
                name: tool_name.clone(),
                arguments: serde_json::to_string(arguments).unwrap_or_default(),
            },
        }]),
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
