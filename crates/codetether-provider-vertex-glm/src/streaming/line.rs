//! One SSE data line, including the terminal marker.
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
        && let Ok(parsed) = serde_json::from_str::<StreamResponse>(data)
        && let Some(choice) = parsed.choices.first()
    {
        if let Some(ref content) = choice.delta.content {
            text_buf.push_str(content);
        }
        tools::push(choice, text_buf, chunks);
        finish::push(choice, text_buf, chunks);
    }
}
