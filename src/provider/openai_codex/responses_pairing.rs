//! Repair interrupted tool groups in request projections, never in saved history.

use std::collections::HashSet;

use serde_json::{Value, json};

pub(super) fn repair(input: Vec<Value>) -> Vec<Value> {
    let fulfilled: HashSet<String> = input
        .iter()
        .filter(|item| item["type"] == "function_call_output")
        .filter_map(|item| item["call_id"].as_str().map(str::to_owned))
        .collect();
    let mut repaired = Vec::with_capacity(input.len());
    for item in input {
        let missing = (item["type"] == "function_call")
            .then(|| item["call_id"].as_str())
            .flatten()
            .filter(|id| !fulfilled.contains(*id))
            .map(str::to_owned);
        repaired.push(item);
        if let Some(id) = missing {
            tracing::warn!(tool_call_id = %id, "Recovering missing tool output in Codex request");
            repaired.push(json!({
                "type": "function_call_output",
                "call_id": id,
                "output": json!({
                    "success": false,
                    "error": "tool_result_unavailable",
                    "message": "No result was retained for this tool call. Its execution outcome is unknown; do not assume success or automatically repeat side effects."
                }).to_string(),
            }));
        }
    }
    repaired
}

#[cfg(test)]
#[path = "responses_pairing_idempotence_tests.rs"]
mod idempotence_tests;
#[cfg(test)]
#[path = "responses_pairing_tests.rs"]
mod tests;
