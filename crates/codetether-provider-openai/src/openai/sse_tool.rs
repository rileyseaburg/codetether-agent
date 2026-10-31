//! Map a streamed tool-call fragment to [`StreamChunk`]s.

use codetether_provider_core::StreamChunk;
use std::collections::HashMap;

use super::sse_types::SseToolCall;

/// Emit chunks for a single tool-call fragment.
pub(super) fn tool_chunks(tc: &SseToolCall, ids: &mut HashMap<usize, String>) -> Vec<StreamChunk> {
    if let Some(id) = &tc.id {
        ids.insert(tc.index, id.clone());
    }
    let mut out = Vec::new();
    let id = tc
        .id
        .clone()
        .or_else(|| ids.get(&tc.index).cloned())
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
