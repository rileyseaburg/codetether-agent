//! `contentBlockDelta` handling for the Bedrock converse-stream adapter.
//!
//! Translates a single delta event into [`StreamChunk`]s:
//! - `delta.text`             → [`StreamChunk::Text`]
//! - `delta.toolUse.input`    → [`StreamChunk::ToolCallDelta`]
//! - `delta.reasoningContent` → [`StreamChunk::Thinking`] for plaintext;
//!   encrypted (signature-only) deltas are logged at debug level so the
//!   invisible token spend of models like Claude Fable 5 is observable.

use super::StreamState;
use crate::provider::StreamChunk;
use serde_json::Value;

/// Convert one `contentBlockDelta` payload into zero or more chunks.
pub(super) fn handle_block_delta(state: &StreamState, body: &Value) -> Vec<StreamChunk> {
    let idx = body.get("contentBlockIndex").and_then(|v| v.as_u64());
    let Some(delta) = body.get("delta") else {
        return Vec::new();
    };

    if let Some(text) = delta.get("text").and_then(|v| v.as_str())
        && !text.is_empty()
    {
        return vec![StreamChunk::Text(text.to_string())];
    }

    if let Some(tool) = delta.get("toolUse")
        && let Some(partial) = tool.get("input").and_then(|v| v.as_str())
        && let Some(idx) = idx
        && let Some(id) = state.open_tool_blocks.get(&idx)
    {
        return vec![StreamChunk::ToolCallDelta {
            id: id.clone(),
            arguments_delta: partial.to_string(),
        }];
    }

    if let Some(reasoning) = delta.get("reasoningContent") {
        if let Some(text) = reasoning.get("text").and_then(|v| v.as_str())
            && !text.is_empty()
        {
            return vec![StreamChunk::Thinking(text.to_string())];
        }
        if let Some(sig) = reasoning.get("signature").and_then(|v| v.as_str()) {
            crate::provider::bedrock::reasoning_audit::record_signature(
                "converse-stream",
                idx,
                sig,
            );
        }
    }

    Vec::new()
}
