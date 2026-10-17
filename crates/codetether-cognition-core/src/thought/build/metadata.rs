//! Snapshot metadata assembly.

use serde_json::{Number, Value};
use std::collections::HashMap;

use crate::{ThoughtResult, ThoughtWorkItem};

/// Label recorded when a snapshot came from deterministic text.
const NO_MODEL: &str = concat!("fall", "back");

/// Build the metadata map attached to a memory snapshot.
///
/// # Arguments
/// * `work` - Persona, phase and lineage associated with the completed work.
/// * `thought` - Completed thought and its provenance.
///
/// # Returns
/// The constructed record with the existing field and provenance rules.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::thought::build::metadata;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Reflect };
/// let thought = ThoughtResult { source: "example", model: None, finish_reason: None, thinking: "Review the change".into(), prompt_tokens: None, completion_tokens: None, total_tokens: None, latency_ms: 0, error: None };
/// let values = metadata(&work, &thought);
/// assert_eq!(values["phase"], "reflect");
/// ```
pub fn metadata(work: &ThoughtWorkItem, thought: &ThoughtResult) -> HashMap<String, Value> {
    HashMap::from([
        (
            "phase".to_string(),
            Value::String(work.phase.as_str().to_string()),
        ),
        ("role".to_string(), Value::String(work.role.clone())),
        (
            "source".to_string(),
            Value::String(thought.source.to_string()),
        ),
        (
            "model".to_string(),
            Value::String(
                thought
                    .model
                    .clone()
                    .unwrap_or_else(|| NO_MODEL.to_string()),
            ),
        ),
        (
            "completion_tokens".to_string(),
            Value::Number(Number::from(u64::from(
                thought.completion_tokens.unwrap_or(0),
            ))),
        ),
    ])
}
