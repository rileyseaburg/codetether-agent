//! State retained while decoding one Responses stream.
use crate::*;

/// Streaming state for a single function-call item.
///
/// # Examples
/// ```
/// let state = codetether_codex_protocol::ResponsesToolState::default();
/// ```
#[derive(Debug, Default)]
pub struct ResponsesToolState {
    pub(crate) call_id: String,
    pub(crate) name: Option<String>,
    pub(crate) started: bool,
    pub(crate) finished: bool,
    pub(crate) emitted_arguments: String,
}

/// Retained SSE buffers and function-call state for one response stream.
///
/// # Examples
/// ```
/// use codetether_codex_protocol::{CodexProtocol, ResponsesSseParser};
/// let mut parser = ResponsesSseParser::default();
/// assert!(CodexProtocol::parse_responses_sse_bytes(&mut parser, b"data: [DONE]\n\n").is_empty());
/// ```
#[derive(Debug, Default)]
pub struct ResponsesSseParser {
    pub(crate) line_buffer: String,
    pub(crate) event_data_lines: Vec<String>,
    pub(crate) tools: HashMap<String, ResponsesToolState>,
}
