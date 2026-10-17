//! TaskUpdate training record projection.
use super::*;
pub(super) fn build(
    task_id: &String,
    state: &TaskState,
    message: &Option<String>,
    meta: TrainingMetadata,
) -> TrainingRecord {
    {
        let msg = message.as_deref().unwrap_or("");
        TrainingRecord {
            role: "system".into(),
            content: Some(format!("Task `{task_id}` → {state:?}. {msg}")),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        }
    }
}
