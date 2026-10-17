//! RalphProgress training record projection.
use super::*;
pub(super) fn build(
    prd_id: &String,
    passed: &usize,
    total: &usize,
    iteration: &usize,
    status: &String,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Ralph progress (PRD {prd_id}): {passed}/{total} stories passed, iter {iteration}, status: {status}"
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
