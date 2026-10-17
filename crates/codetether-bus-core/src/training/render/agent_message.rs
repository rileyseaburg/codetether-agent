//! AgentMessage training record projection.
use super::*;
pub(super) fn build(
    from: &String,
    to: &String,
    parts: &Vec<Part>,
    meta: TrainingMetadata,
) -> TrainingRecord {
    {
        let text = parts_to_text(parts);
        TrainingRecord {
            role: "assistant".into(),
            content: Some(agent_message_text(from, to, &text)),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        }
    }
}
