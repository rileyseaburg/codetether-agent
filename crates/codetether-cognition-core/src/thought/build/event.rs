//! Thought event payload construction for one tick.

use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

use crate::{ThoughtEvent, ThoughtResult, ThoughtWorkItem};

/// Build the `thought_generated`-family event for a completed thought.
///
/// # Arguments
/// * `work` - Persona, phase and lineage associated with the completed work.
/// * `thought` - Completed thought and its provenance.
/// * `context_event_count` - Number of context events supplied to the thought.
/// * `timestamp` - Timestamp assigned to the emitted event.
///
/// # Returns
/// The constructed record with the existing field and provenance rules.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::thought::build::thought_event;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Reflect };
/// let thought = ThoughtResult { source: "example", model: None, finish_reason: None, thinking: "Review the change".into(), prompt_tokens: None, completion_tokens: None, total_tokens: None, latency_ms: 0, error: None };
/// let event = thought_event(&work, &thought, 2, chrono::Utc::now());
/// assert_eq!(event.payload["context_event_count"], 2);
/// ```
pub fn thought_event(
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
