use crate::{ThoughtEvent, ThoughtEventType, ThoughtResult, ThoughtWorkItem};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;
/// Emit the check-result event for a completed Test-phase thought.
pub(super) fn check_result_event(work: &ThoughtWorkItem, thought: &ThoughtResult) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp: Utc::now(),
        payload: json!({
            "phase": work.phase.as_str(),
            "thought_count": work.thought_count,
            "result_excerpt": crate::text::trim_for_storage(&thought.thinking, 280),
            "source": thought.source,
            "model": thought.model,
        }),
    }
}
