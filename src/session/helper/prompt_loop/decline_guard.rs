//! Per-turn memory for tool invocations explicitly declined by the user.

use serde_json::Value;
use std::collections::HashMap;

const DENIED_CODE: &str = "TOOL_APPROVAL_DENIED";

/// Remembers exact tool invocations declined during one prompt-loop turn.
///
/// A new guard is created for each user turn, so later turns may ask again.
#[derive(Default)]
pub(super) struct DeclineGuard {
    denied: Vec<(String, Value)>,
}

impl DeclineGuard {
    /// Returns model-facing guidance when an invocation was already declined.
    ///
    /// # Arguments
    ///
    /// * `tool` — Canonical tool name requested by the model.
    /// * `input` — Parsed tool arguments before runtime enrichment.
    ///
    /// # Returns
    ///
    /// A blocking explanation for a repeated decline, otherwise `None`.
    pub(super) fn blocked(&self, tool: &str, input: &Value) -> Option<String> {
        self.denied
            .iter()
            .any(|denied| denied == &(tool.to_string(), normalized(input)))
            .then(|| {
                "The user already declined this exact tool request during the current turn. \
                 Do not request it again, even after unrelated calls. Follow the user's \
                 feedback and choose a different approach or report the blocker."
                    .into()
            })
    }

    /// Records an invocation when its result is an explicit approval denial.
    ///
    /// # Arguments
    ///
    /// * `tool` — Canonical tool name requested by the model.
    /// * `input` — Parsed tool arguments before runtime enrichment.
    /// * `metadata` — Structured result metadata used to identify denials.
    pub(super) fn record(
        &mut self,
        tool: &str,
        input: &Value,
        metadata: Option<&HashMap<String, Value>>,
    ) {
        let denied = metadata
            .and_then(|values| values.get("error_code"))
            .and_then(Value::as_str)
            == Some(DENIED_CODE);
        let invocation = (tool.to_string(), normalized(input));
        if denied && !self.denied.contains(&invocation) {
            self.denied.push(invocation);
        }
    }
}

fn normalized(input: &Value) -> Value {
    let mut value = input.clone();
    if let Some(values) = value.as_object_mut() {
        for field in ["approval_id", "_tool_call_id", "justification"] {
            values.remove(field);
        }
    }
    value
}
