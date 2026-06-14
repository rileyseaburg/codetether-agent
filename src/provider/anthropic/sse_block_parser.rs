//! SSE content-block parsing for Anthropic streaming events.
//!
//! Handles `content_block_start`, `content_block_delta`, and tool-call ID
//! tracking for Anthropic SSE streams.

use serde_json::Value;

use crate::provider::StreamChunk;

/// Tracks the active tool-call ID across content-block delta events.
pub(crate) struct BlockParser {
    tool_id: Option<String>,
}

impl BlockParser {
    /// Create a new block parser with no active tool call.
    pub(crate) fn new() -> Self {
        Self { tool_id: None }
    }

    /// Handle a `content_block_start` event and return a chunk if applicable.
    pub(crate) fn start(&mut self, event: &Value) -> Option<StreamChunk> {
        let block = event.get("content_block")?;
        if block.get("type")?.as_str()? == "tool_use" {
            let id = block.get("id")?.as_str()?.to_string();
            let name = block.get("name")?.as_str()?.to_string();
            self.tool_id = Some(id.clone());
            return Some(StreamChunk::ToolCallStart { id, name });
        }
        None
    }

    /// Handle a `content_block_delta` event and return a chunk if applicable.
    pub(crate) fn delta(&mut self, event: &Value) -> Option<StreamChunk> {
        let delta = event.get("delta")?;
        match delta.get("type")?.as_str()? {
            "text_delta" => Some(StreamChunk::Text(delta.get("text")?.as_str()?.to_string())),
            "thinking_delta" => Some(StreamChunk::Thinking(
                delta.get("thinking")?.as_str()?.to_string(),
            )),
            "input_json_delta" => {
                let args = delta
                    .get("partial_json")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                Some(StreamChunk::ToolCallDelta {
                    id: self.tool_id.clone().unwrap_or_default(),
                    arguments_delta: args,
                })
            }
            _ => None,
        }
    }
}
