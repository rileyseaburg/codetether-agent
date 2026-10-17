//! SSE data lines and usage-only events.
use super::{finish, tools};
use crate::*;
pub(super) fn line(line: &str, text_buf: &mut String, chunks: &mut Vec<StreamChunk>) {
    if line == "data: [DONE]" {
        if !text_buf.is_empty() {
            chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
        }
        chunks.push(StreamChunk::Done { usage: None });
        return;
    }

    if let Some(data) = line.strip_prefix("data: ")
        && let Ok(parsed) = serde_json::from_str::<Glm5StreamResponse>(data)
    {
        // Capture usage from the final chunk (stream_options)
        let usage = parsed.usage.as_ref().map(Glm5Usage::to_usage);

        if let Some(choice) = parsed.choices.first() {
            if let Some(ref content) = choice.delta.content {
                text_buf.push_str(content);
            }

            // Streaming tool calls
            tools::push(choice, text_buf, chunks);

            finish::push(choice, usage, text_buf, chunks);
        } else if usage.is_some() {
            // Usage-only chunk (empty choices)
            if !text_buf.is_empty() {
                chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
            }
            chunks.push(StreamChunk::Done { usage });
        }
    }
}
