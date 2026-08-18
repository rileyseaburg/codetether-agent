//! Thought event payload construction for one tick.

use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

use super::{ThoughtEvent, ThoughtResult, ThoughtWorkItem};

/// Build the `thought_generated`-family event for a completed thought.
pub(super) fn thought_event(
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    context_event_count: usize,
    timestamp: DateTime<Utc>,
) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: work.phase.event_type(),
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp,
        payload: json!({
            "phase": work.phase.as_str(),
            "thought_count": work.thought_count,
            "persona": {
                "id": work.persona_id.clone(),
                "name": work.persona_name.clone(),
                "role": work.role.clone(),
            },
            "context_event_count": context_event_count,
            "thinking": thought.thinking.clone(),
            "source": thought.source,
            "model": thought.model.clone(),
            "finish_reason": thought.finish_reason.clone(),
            "usage": {
                "prompt_tokens": thought.prompt_tokens,
                "completion_tokens": thought.completion_tokens,
                "total_tokens": thought.total_tokens,
            },
            "latency_ms": thought.latency_ms,
            "error": thought.error.clone(),
        }),
    }
}
