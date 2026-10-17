//! RalphLearning training record projection.
use super::*;
pub(super) fn build(
    prd_id: &String,
    story_id: &String,
    iteration: &usize,
    learnings: &Vec<String>,
    context: &serde_json::Value,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Ralph learning (PRD {prd_id}, story {story_id}, iter {iteration}):\n{}\nContext: {}",
            learnings
                .iter()
                .map(|l| format!("- {l}"))
                .collect::<Vec<_>>()
                .join("\n"),
            serde_json::to_string(context).unwrap_or_default()
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
