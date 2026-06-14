//! Parse `message_delta` SSE events into [`StreamChunk::Done`].

use serde_json::Value;

use crate::provider::{StreamChunk, Usage};

/// Extract the done chunk from a `message_delta` event.
///
/// Maps `usage.output_tokens` into a provider-neutral [`Usage`] struct.
pub(crate) fn parse(event: &Value) -> StreamChunk {
    let usage = event.get("usage").map(|u| {
        let out = u.get("output_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
        Usage {
            completion_tokens: out as usize,
            ..Default::default()
        }
    });
    StreamChunk::Done { usage }
}
