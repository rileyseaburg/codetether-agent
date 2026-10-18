//! Stateful provider-chunk encoding for OpenAI-compatible SSE responses.
//!
//! Encoder owns stream metadata, tool indexes and final reason selection.
//! Emission tells the transport whether to skip, send, or terminate after an error.
use std::collections::HashMap;
mod dispatch;
mod event;
mod lifecycle;
mod text;
mod tool_delta;
mod tool_index;
mod tool_start;
/// Incremental OpenAI stream encoder.
/// # Examples
/// ```
/// let _encoder = codetether_openai_stream::Encoder::new("id".into(), 0, "model".into());
/// ```
pub struct Encoder {
    id: String,
    created: i64,
    model: String,
    tool_call_indices: HashMap<String, usize>,
    next_tool_call_index: usize,
    saw_text: bool,
    saw_tool_calls: bool,
}
/// Action the SSE transport takes after encoding a chunk.
pub enum Emission {
    /// No visible event for this chunk.
    Skip,
    /// Send a normal event.
    Event(axum::response::sse::Event),
    /// Send the error event followed by the transport's DONE marker.
    Error(axum::response::sse::Event),
}
