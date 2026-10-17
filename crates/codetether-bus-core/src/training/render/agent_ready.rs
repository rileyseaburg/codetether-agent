//! AgentReady training record projection.
use super::*;
pub(super) fn build(
    agent_id: &String,
    capabilities: &Vec<String>,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Agent `{agent_id}` ready. Capabilities: {}",
            capabilities.join(", ")
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
