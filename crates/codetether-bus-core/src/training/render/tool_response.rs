//! ToolResponse training record projection.
use super::*;
pub(super) fn build(
    request_id: &String,
    tool_name: &String,
    result: &String,
    success: &bool,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "tool".into(),
        content: Some(if *success {
            result.clone()
        } else {
            format!("[ERROR] {result}")
        }),
        tool_calls: None,
        tool_call_id: Some(request_id.clone()),
        name: Some(tool_name.clone()),
        metadata: meta,
    }
}
