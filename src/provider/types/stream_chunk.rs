//! Provider-neutral streaming events.

use super::{ContentPart, Usage};

/// A streaming chunk produced by [`Provider::complete_stream`](crate::provider::Provider::complete_stream).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::StreamChunk;
/// let chunk = StreamChunk::Text("hello".into());
/// assert!(matches!(chunk, StreamChunk::Text(_)));
/// ```
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Transport activity without model output.
    KeepAlive,
    /// Incremental text delta.
    Text(String),
    /// Model thinking/reasoning content.
    Thinking(String),
    /// Beginning of a tool call.
    ToolCallStart { id: String, name: String },
    /// Partial tool-call arguments.
    ToolCallDelta { id: String, arguments_delta: String },
    /// End of a tool call.
    ToolCallEnd { id: String },
    /// One complete Responses output item that can survive a later disconnect.
    OutputItemDone { content: ContentPart },
    /// Stream finished.
    Done { usage: Option<Usage> },
    /// Terminal stream error.
    Error(String),
}
