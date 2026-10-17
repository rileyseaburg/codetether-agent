//! Text flush, tool ending, and completion event.
use crate::*;
pub(super) fn push(
    choice: &Glm5StreamChoice,
    usage: Option<Usage>,
    text_buf: &mut String,
    chunks: &mut Vec<StreamChunk>,
) {
    if let Some(ref reason) = choice.finish_reason {
        if !text_buf.is_empty() {
            chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
        }
        if reason == "tool_calls"
            && let Some(ref tcs) = choice.delta.tool_calls
            && let Some(tc) = tcs.last()
        {
            chunks.push(StreamChunk::ToolCallEnd {
                id: tc.id.clone().unwrap_or_default(),
            });
        }
        chunks.push(StreamChunk::Done { usage });
    }
}
