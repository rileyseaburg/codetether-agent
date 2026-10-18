//! Map every provider chunk to a transport action.
use crate::{Emission, Encoder};
use codetether_openai_wire::{openai_stream_chunk, stream::reasoning_delta};
use codetether_provider_core::StreamChunk as Chunk;
impl Encoder {
    /// Encode one provider chunk, updating tool indexes and final-reason flags.
    /// # Arguments
    /// * `chunk` - Provider event to encode.
    /// # Returns
    /// Skip, a normal SSE event, or a terminal error event.
    /// # Examples
    /// ```
    /// let mut encoder = codetether_openai_stream::Encoder::new("id".into(), 0, "model".into());
    /// assert!(matches!(encoder.push(codetether_provider_core::StreamChunk::KeepAlive), codetether_openai_stream::Emission::Skip));
    /// ```
    pub fn push(&mut self, chunk: Chunk) -> Emission {
        match chunk {
            Chunk::Text(text) => self.text(text),
            Chunk::ToolCallStart { id, name } => self.tool_start(id, name),
            Chunk::ToolCallDelta {
                id,
                arguments_delta,
            } => self.tool_delta(id, arguments_delta),
            Chunk::Thinking(text) if !text.is_empty() => {
                Emission::Event(crate::event::event(&openai_stream_chunk(
                    &self.id,
                    self.created,
                    &self.model,
                    reasoning_delta(text),
                    None,
                )))
            }
            Chunk::KeepAlive
            | Chunk::ToolCallEnd { .. }
            | Chunk::OutputItemDone { .. }
            | Chunk::Done { .. }
            | Chunk::Thinking(_) => Emission::Skip,
            Chunk::Error(error) => Emission::Error(crate::event::event(&openai_stream_chunk(
                &self.id,
                self.created,
                &self.model,
                serde_json::json!({"content":error}),
                Some("error"),
            ))),
        }
    }
}
