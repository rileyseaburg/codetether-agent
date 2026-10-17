//! Heartbeat training record projection.
use super::*;
pub(super) fn build(meta: TrainingMetadata) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: None,
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
