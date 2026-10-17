//! In-place repair of unpaired toolUse blocks in a final request body.

use super::insert::insert_results;
use serde_json::Value;

/// Ensure every `toolUse` in `body["messages"]` has a following `toolResult`.
///
/// Returns the ids that had to be synthesized, so callers can log the fact
/// that a malformed body was caught locally rather than by Bedrock.
///
/// # Arguments
/// * `body` - Final Converse request JSON to inspect and repair in place.
///
/// # Returns
/// Identifiers of tool results synthesized by the repair.
///
/// # Examples
/// ```
/// let mut body = serde_json::json!({"messages": []});
/// assert!(codetether_bedrock_protocol::audit::enforce(&mut body).is_empty());
/// ```
pub fn enforce(body: &mut Value) -> Vec<String> {
    let Some(messages) = body.get_mut("messages").and_then(Value::as_array_mut) else {
        return Vec::new();
    };
    let mut synthesized = Vec::new();
    while let Some((index, missing)) = super::scan::unpaired(messages) {
        insert_results(messages, index + 1, &missing);
        synthesized.extend(missing);
    }
    if !synthesized.is_empty() {
        tracing::warn!(
            provider = "bedrock",
            ids = ?synthesized,
            "repaired unpaired toolUse blocks before send"
        );
    }
    synthesized
}
