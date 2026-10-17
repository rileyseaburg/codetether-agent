//! ToolOutputFull training record projection.
use super::*;
pub(super) fn build(
    agent_id: &String,
    tool_name: &String,
    output: &String,
    success: &bool,
    step: &usize,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "tool".into(),
        content: Some(if *success {
            format!("[step {step}, agent {agent_id}] {output}")
        } else {
            format!("[step {step}, agent {agent_id}, ERROR] {output}")
        }),
        tool_calls: None,
        tool_call_id: None,
        name: Some(tool_name.clone()),
        metadata: meta,
    }
}
