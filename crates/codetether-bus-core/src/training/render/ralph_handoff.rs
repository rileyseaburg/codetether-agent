//! RalphHandoff training record projection.
use super::*;
pub(super) fn build(
    prd_id: &String,
    from_story: &String,
    to_story: &String,
    context: &serde_json::Value,
    progress_summary: &String,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Ralph handoff (PRD {prd_id}): {from_story} → {to_story}\nSummary: {progress_summary}\nContext: {}",
            serde_json::to_string(context).unwrap_or_default()
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
