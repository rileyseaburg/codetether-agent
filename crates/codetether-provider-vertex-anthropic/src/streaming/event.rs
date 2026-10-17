//! Existing event-kind dispatch.
use crate::*;
pub(super) fn dispatch(event: &Value, chunks: &mut Vec<StreamChunk>, text_buf: &mut String) {
    let event_type = event.get("type").and_then(Value::as_str).unwrap_or("");

    match event_type {
        "content_block_delta" => {
            if let Some(delta) = event.get("delta")
                && let Some(text) = delta.get("text").and_then(Value::as_str)
            {
                text_buf.push_str(text);
            }
        }
        "content_block_start" => {
            // Tool call start
            if let Some(content_block) = event.get("content_block")
                && content_block.get("type").and_then(Value::as_str) == Some("tool_use")
            {
                if !text_buf.is_empty() {
                    chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
                }
                let id = content_block
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let name = content_block
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                chunks.push(StreamChunk::ToolCallStart {
                    id: id.to_string(),
                    name: name.to_string(),
                });
            }
        }
        "content_block_stop" => {
            // Tool call end
            let index = event.get("index").and_then(Value::as_u64);
            if let Some(_idx) = index {
                // We'd need to track tool call IDs, for now emit end
                // This is a simplification - real impl needs state tracking
            }
        }
        "message_delta" => {
            // Final stop reason
            if let Some(_usage) = event.get("usage") {
                // Could extract usage here
            }
        }
        "message_stop" => {
            if !text_buf.is_empty() {
                chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
            }
            chunks.push(StreamChunk::Done { usage: None });
        }
        _ => {}
    }
}
