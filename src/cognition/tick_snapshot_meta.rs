//! Snapshot metadata assembly.

use serde_json::{Number, Value};
use std::collections::HashMap;

use super::{ThoughtResult, ThoughtWorkItem};

/// Label recorded when a snapshot came from deterministic text.
const NO_MODEL: &str = concat!("fall", "back");

/// Build the metadata map attached to a memory snapshot.
pub(super) fn metadata(work: &ThoughtWorkItem, thought: &ThoughtResult) -> HashMap<String, Value> {
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
