//! Stream initialization and terminal reason projection.
use crate::Encoder;
use axum::response::sse::Event;
use codetether_openai_wire::{openai_stream_chunk, stream::finish_reason};
impl Encoder {
    /// Create an encoder with empty tool indexes and output flags.
    /// # Examples
    /// ```
    /// let _ = codetether_openai_stream::Encoder::new("id".into(), 0, "model".into());
    /// ```
    pub fn new(id: String, created: i64, model: String) -> Self {
        Self {
            id,
            created,
            model,
            tool_call_indices: Default::default(),
            next_tool_call_index: 0,
            saw_text: false,
            saw_tool_calls: false,
        }
    }
    /// Build the initial assistant-role event.
    /// # Examples
    /// ```
    /// let encoder = codetether_openai_stream::Encoder::new("id".into(), 0, "model".into());
    /// let _ = encoder.start();
    /// ```
    pub fn start(&self) -> Event {
        crate::event::event(&openai_stream_chunk(
            &self.id,
            self.created,
            &self.model,
            serde_json::json!({"role":"assistant"}),
            None,
        ))
    }
    /// Build a terminal event using the observed text and tool-call flags.
    /// # Examples
    /// ```
    /// let encoder = codetether_openai_stream::Encoder::new("id".into(), 0, "model".into());
    /// let _ = encoder.finish();
    /// ```
    pub fn finish(&self) -> Event {
        crate::event::event(&openai_stream_chunk(
            &self.id,
            self.created,
            &self.model,
            serde_json::json!({}),
            Some(finish_reason(self.saw_tool_calls, self.saw_text)),
        ))
    }
}
