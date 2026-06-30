//! Map one parsed SSE frame to [`StreamChunk`]s for the raw reasoning stream.

use crate::provider::{StreamChunk, Usage};

use super::sse_tool::tool_chunks;
use super::sse_types::{SseResponse, SseUsage};

/// Convert a usage frame to the internal [`Usage`] type.
pub(super) fn to_usage(u: &SseUsage) -> Usage {
    Usage {
        prompt_tokens: u.prompt_tokens,
        completion_tokens: u.completion_tokens,
        total_tokens: u.total_tokens,
        cache_read_tokens: None,
        cache_write_tokens: None,
    }
}

/// Map a full SSE frame to chunks, recording whether output was produced.
pub(super) fn frame_chunks(parsed: &SseResponse, produced: &mut bool) -> Vec<StreamChunk> {
    let mut out = Vec::new();
    let Some(choice) = parsed.choices.first() else {
        return out;
    };
    if let Some(r) = &choice.delta.reasoning_content
        && !r.is_empty()
    {
        *produced = true;
        out.push(StreamChunk::Thinking(r.clone()));
    }
    if let Some(c) = &choice.delta.content
        && !c.is_empty()
    {
        *produced = true;
        out.push(StreamChunk::Text(c.clone()));
    }
    if let Some(tcs) = &choice.delta.tool_calls {
        for tc in tcs {
            let chunks = tool_chunks(tc);
            if !chunks.is_empty() {
                *produced = true;
            }
            out.extend(chunks);
        }
    }
    out
}
