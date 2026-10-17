//! UserPrompt training record projection.
use super::*;
pub(super) fn build(
    agent_id: &String,
    text: &String,
    session_id: &String,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "user".into(),
        content: Some(text.clone()),
        tool_calls: None,
        tool_call_id: None,
        name: Some(format!("user.{agent_id}.{session_id}")),
        metadata: meta,
    }
}
