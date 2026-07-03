//! Per-turn repeat-call guard for edit-family tools (issue #294).
//!
//! The existing `consecutive_same_tool` loop guard compares the *entire*
//! tool-call batch signature.  When the model interleaves a read with the
//! same failing edit, the batch sig changes and the counter resets, so the
//! model can re-issue an identical edit indefinitely.
//!
//! This guard tracks individual edit-family tool calls by their content
//! signature (`name` + canonical `args`). It allows up to `THRESHOLD`
//! identical attempts within a single turn and blocks every attempt after
//! that (i.e. the `THRESHOLD + 1`-th call and beyond), injecting a clear
//! message that tells the model to stop retrying.

use serde_json::Value;
use std::collections::HashMap;

/// Edit-family tools whose repetitions we guard against.
const EDIT_TOOLS: &[&str] = &[
    "edit",
    "write",
    "multiedit",
    "patch",
    "apply_patch",
    "confirm_edit",
];

/// Identical attempts allowed before the guard fires on the next one.
const THRESHOLD: u8 = 3;

/// Tracks per-signature call counts for the duration of one agentic turn.
#[derive(Default)]
pub struct RepeatGuard {
    counts: HashMap<String, u8>,
}

impl RepeatGuard {
    /// Returns `Some(reason)` when the tool call should be blocked.
    pub fn check(&mut self, tool_name: &str, tool_input: &Value) -> Option<String> {
        if !EDIT_TOOLS.contains(&tool_name) {
            return None;
        }
        let sig = signature(tool_name, tool_input);
        let count = self.counts.entry(sig).or_insert(0);
        *count += 1;
        (*count > THRESHOLD).then(|| {
            "You have already attempted this exact edit multiple times and it was \
             not applied. Stop retrying the identical edit — the target text may \
             have changed or the edit may not be applicable. Re-read the file to \
             see its current content, then adjust your edit or report the blocker."
                .to_string()
        })
    }
}

fn signature(tool_name: &str, input: &Value) -> String {
    let canonical = canonicalize(input);
    format!("{tool_name}:{canonical}")
}

/// Produce a stable string key from tool input, ignoring volatile fields
/// like `tool_call_id` or `instruction` that change per call.
fn canonicalize(input: &Value) -> String {
    let filtered = filter_volatile(input);
    serde_json::to_string(&filtered).unwrap_or_else(|_| input.to_string())
}

fn filter_volatile(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if matches!(k.as_str(), "tool_call_id" | "instruction" | "update") {
                    continue;
                }
                out.insert(k.clone(), filter_volatile(v));
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(filter_volatile).collect()),
        _ => value.clone(),
    }
}
