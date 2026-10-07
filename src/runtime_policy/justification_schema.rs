//! Advertise `justification` as required in tool schemas while in `ask` mode.
//!
//! The runtime already refuses an `ask`-mode approval prompt until the model
//! supplies a justification. Marking the field required up front lets the
//! model include it on the first call instead of learning it from a refusal,
//! saving a wasted round trip per mutating call.

use super::ToolKind;
use crate::config::AccessMode;
use crate::provider::ToolDefinition;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};

static ASK_MODE: AtomicBool = AtomicBool::new(false);

/// Record the effective access mode used when advertising tool schemas.
pub fn note_access_mode(mode: Option<AccessMode>) {
    ASK_MODE.store(mode == Some(AccessMode::Ask), Ordering::Relaxed);
}

/// Mark `justification` required on mutating tools when `ask` mode is active.
pub(crate) fn apply(definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    if !ASK_MODE.load(Ordering::Relaxed) {
        return definitions;
    }
    definitions.into_iter().map(require).collect()
}

fn require(mut definition: ToolDefinition) -> ToolDefinition {
    let field = super::justification::FIELD;
    let params = &mut definition.parameters;
    let mutating = ToolKind::for_name(&definition.name) == ToolKind::Mutating;
    if !mutating || params.pointer(&format!("/properties/{field}")).is_none() {
        return definition;
    }
    match params.get_mut("required") {
        Some(Value::Array(required)) if !required.iter().any(|v| v == field) => {
            required.push(json!(field));
        }
        Some(_) => {}
        None => params["required"] = json!([field]),
    }
    definition
}

#[cfg(test)]
#[path = "justification_schema_tests.rs"]
mod tests;
