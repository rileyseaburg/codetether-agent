//! `contentBlockStart` / `contentBlockStop` / `metadata` event handlers for
//! the Bedrock converse-stream adapter.
//!
//! Tool-use block lifecycle is tracked in [`StreamState`] so argument deltas
//! and `ToolCallEnd` events can be correlated by `contentBlockIndex`.

use super::StreamState;
use crate::provider::{StreamChunk, Usage};
use serde_json::Value;

/// Open a tool-use block and emit [`StreamChunk::ToolCallStart`].
pub(super) fn handle_block_start(state: &mut StreamState, body: &Value) -> Vec<StreamChunk> {
    let Some(idx) = body.get("contentBlockIndex").and_then(|v| v.as_u64()) else {
        return Vec::new();
    };
    let Some(tool_use) = body.get("start").and_then(|v| v.get("toolUse")) else {
        return Vec::new();
    };
    let field = |key: &str| {
        tool_use
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let id = field("toolUseId");
    let name = field("name");
    if id.is_empty() {
        return Vec::new();
    }
    state.open_tool_blocks.insert(idx, id.clone());
    vec![StreamChunk::ToolCallStart { id, name }]
}

/// Close an open tool-use block and emit [`StreamChunk::ToolCallEnd`].
pub(super) fn handle_block_stop(state: &mut StreamState, body: &Value) -> Vec<StreamChunk> {
    let Some(idx) = body.get("contentBlockIndex").and_then(|v| v.as_u64()) else {
        return Vec::new();
    };
    if let Some(id) = state.open_tool_blocks.remove(&idx) {
        return vec![StreamChunk::ToolCallEnd { id }];
    }
    Vec::new()
}

/// Parse the `metadata` event's usage block.
pub(super) fn extract_usage(body: &Value) -> Option<Usage> {
    let u = body.get("usage")?;
    Some(Usage {
        prompt_tokens: u.get("inputTokens").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        completion_tokens: u.get("outputTokens").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        total_tokens: u.get("totalTokens").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        cache_read_tokens: u
            .get("cacheReadInputTokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize),
        cache_write_tokens: u
            .get("cacheWriteInputTokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize),
    })
}
