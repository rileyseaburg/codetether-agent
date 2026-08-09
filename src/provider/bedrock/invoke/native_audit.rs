//! Pairing audit for native Anthropic Messages bodies (InvokeModel path).
//!
//! The Converse audit matches `toolUse`/`toolResult`; the native body uses
//! `type`-tagged `tool_use`/`tool_result` blocks with `id`/`tool_use_id`, so
//! it needs its own pass after [`super::invoke_msgconvert`] remapping.

use serde_json::Value;

#[path = "native_audit/insert.rs"]
mod insert;
#[path = "native_audit/scan.rs"]
mod scan;

/// Ensure every native `tool_use` block is answered in the next message.
///
/// Returns the ids that had to be synthesized.
pub(super) fn enforce_native(body: &mut Value) -> Vec<String> {
    let Some(messages) = body.get_mut("messages").and_then(Value::as_array_mut) else {
        return Vec::new();
    };
    let mut synthesized = Vec::new();
    while let Some((index, missing)) = scan::unpaired(messages) {
        insert::results(messages, index + 1, &missing);
        synthesized.extend(missing);
    }
    if !synthesized.is_empty() {
        tracing::warn!(
            provider = "bedrock",
            path = "invoke_model",
            ids = ?synthesized,
            "repaired unpaired tool_use blocks before send"
        );
    }
    synthesized
}

#[cfg(test)]
#[path = "native_audit_tests.rs"]
mod tests;
