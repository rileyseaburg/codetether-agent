//! Streaming request construction and HTTP error shaping for Z.AI.
//!
//! Split out of `zai.rs` so the streaming state machine and its request setup
//! are separate concerns, and so `zai.rs` stays within the line-budget ratchet.

use crate::provider::{CompletionRequest, StreamChunk, Usage};
use serde_json::{Value, json};

/// Builds the JSON body for a streaming chat completion.
///
/// `thinking` is supplied by the caller because the policy lives with the
/// provider; this module only assembles the payload.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::stream_request::build_body;
/// use codetether_agent::provider::CompletionRequest;
/// use serde_json::json;
///
/// let request = CompletionRequest {
///     messages: vec![],
///     tools: vec![],
///     model: "glm-5.2".into(),
///     temperature: None,
///     top_p: None,
///     max_tokens: Some(256),
///     stop: vec![],
/// };
/// let body = build_body(&request, vec![], vec![], json!({"type": "enabled"}), true);
///
/// assert_eq!(body["model"], "glm-5.2");
/// assert_eq!(body["stream"], true);
/// assert_eq!(body["temperature"], 1.0);
/// assert_eq!(body["max_tokens"], 256);
/// ```
pub fn build_body(
    request: &CompletionRequest,
    messages: Vec<Value>,
    tools: Vec<Value>,
    thinking: Value,
    tool_stream: bool,
) -> Value {
    let mut body = json!({
        "model": request.model,
        "messages": messages,
        "temperature": request.temperature.unwrap_or(1.0),
        "stream": true,
    });
    body["thinking"] = thinking;
    if !tools.is_empty() {
        body["tools"] = json!(tools);
        if tool_stream {
            body["tool_stream"] = json!(true);
        }
    }
    if let Some(max) = request.max_tokens {
        body["max_tokens"] = json!(max);
    }
    body
}

/// Renders a non-success HTTP response as terminal stream chunks.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::stream_request::error_chunks;
/// use codetether_agent::provider::StreamChunk;
///
/// let chunks = error_chunks("Z.AI API error: 429 rate limited".into());
///
/// assert!(matches!(&chunks[0], StreamChunk::Error(m) if m.contains("429")));
/// assert!(matches!(chunks[1], StreamChunk::Done { .. }));
/// ```
pub fn error_chunks(message: String) -> Vec<StreamChunk> {
    vec![
        StreamChunk::Error(message),
        StreamChunk::Done {
            usage: None::<Usage>,
        },
    ]
}

#[cfg(test)]
#[path = "zai_stream_request_tests.rs"]
mod tests;
