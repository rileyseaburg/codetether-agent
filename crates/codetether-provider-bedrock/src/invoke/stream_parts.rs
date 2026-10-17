//! Replay buffered native content as provider stream chunks.
use codetether_provider_core::{ContentPart, StreamChunk};

pub(super) fn push_part_chunks(part: ContentPart, chunks: &mut Vec<StreamChunk>) {
    match part {
        ContentPart::Text { text } if !text.is_empty() => chunks.push(StreamChunk::Text(text)),
        ContentPart::Thinking { text, .. } if !text.is_empty() => {
            chunks.push(StreamChunk::Thinking(text));
        }
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => {
            chunks.push(StreamChunk::ToolCallStart {
                id: id.clone(),
                name,
            });
            if !arguments.is_empty() {
                chunks.push(StreamChunk::ToolCallDelta {
                    id: id.clone(),
                    arguments_delta: arguments,
                });
            }
            chunks.push(StreamChunk::ToolCallEnd { id });
        }
        _ => {}
    }
}
