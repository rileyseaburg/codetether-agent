//! Line-buffer draining for the raw reasoning SSE stream.

use crate::provider::{StreamChunk, Usage};

use super::sse_frame::{frame_chunks, to_usage};
use super::sse_types::SseResponse;

/// Consume complete lines from `buffer`, appending chunks to `out`.
pub(super) fn drain_lines(
    buffer: &mut String,
    produced: &mut bool,
    usage: &mut Option<Usage>,
    out: &mut Vec<StreamChunk>,
) {
    while let Some(end) = buffer.find('\n') {
        let line = buffer[..end].trim().to_string();
        *buffer = buffer[end + 1..].to_string();
        if line == "data: [DONE]" {
            if !*produced {
                out.push(StreamChunk::Error(
                    "stream ended without producing any content (empty response)".to_string(),
                ));
            }
            out.push(StreamChunk::Done {
                usage: usage.take(),
            });
            continue;
        }
        let Some(data) = line.strip_prefix("data: ") else {
            continue;
        };
        let Ok(parsed) = serde_json::from_str::<SseResponse>(data) else {
            continue;
        };
        if let Some(u) = &parsed.usage {
            *usage = Some(to_usage(u));
        }
        out.extend(frame_chunks(&parsed, produced));
    }
}
