//! AgentShutdown training record projection.
use super::*;
pub(super) fn build(agent_id: &String, meta: TrainingMetadata) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!("Agent `{agent_id}` shutting down.")),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
