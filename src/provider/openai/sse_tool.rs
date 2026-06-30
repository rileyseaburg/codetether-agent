//! Map a streamed tool-call fragment to [`StreamChunk`]s.

use crate::provider::StreamChunk;

use super::sse_types::SseToolCall;

/// Emit chunks for a single tool-call fragment.
pub(super) fn tool_chunks(tc: &SseToolCall) -> Vec<StreamChunk> {
    let mut out = Vec::new();
    let id = tc
        .id
        .clone()
        .unwrap_or_else(|| format!("tool_{}", tc.index));
    let Some(func) = &tc.function else {
        return out;
    };
    if tc.id.is_some() {
        out.push(StreamChunk::ToolCallStart {
            id: id.clone(),
            name: func.name.clone().unwrap_or_default(),
        });
    }
    if let Some(args) = &func.arguments {
        let delta = match args {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        if !delta.is_empty() {
            out.push(StreamChunk::ToolCallDelta {
                id,
                arguments_delta: delta,
            });
        }
    }
    out
}
