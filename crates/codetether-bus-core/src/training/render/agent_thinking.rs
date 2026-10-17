//! AgentThinking training record projection.
use super::*;
pub(super) fn build(
    agent_id: &String,
    thinking: &String,
    step: &usize,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "assistant".into(),
        content: Some(format!("<thinking>\n{thinking}\n</thinking>")),
        tool_calls: None,
        tool_call_id: None,
        name: Some(format!("reasoning.{agent_id}.step_{step}")),
        metadata: meta,
    }
}
