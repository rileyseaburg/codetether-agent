//! Shared chat-completions request-body construction.
//!
//! `complete` and `complete_stream` previously duplicated this assembly, which
//! let the two paths drift — reasoning had to be added twice. One builder keeps
//! streaming and non-streaming turns identical apart from the `stream` flag.

#[path = "request_number.rs"]
mod number;

use super::reasoning_request;
use super::runtime_config;
use crate::provider::CompletionRequest;
use serde_json::{Value, json};

/// Build the request body for a chat-completions turn.
///
/// # Arguments
///
/// * `request` — Resolved completion request.
/// * `messages` — Provider-encoded messages.
/// * `tools` — Provider-encoded tool definitions; omitted when empty.
/// * `streaming` — Whether to set `stream: true`.
pub(super) fn build(
    request: &CompletionRequest,
    messages: Vec<Value>,
    tools: Vec<Value>,
    streaming: bool,
) -> Value {
    let mut body = json!({ "model": request.model, "messages": messages });
    if streaming {
        body["stream"] = json!(true);
    }
    if !tools.is_empty() {
        body["tools"] = json!(tools);
    }
    if let Some(temperature) = request.temperature {
        body["temperature"] = json!(number::temperature(temperature));
    }
    if let Some(max_tokens) = request.max_tokens {
        body["max_tokens"] = json!(max_tokens);
    }
    reasoning_request::apply(
        &mut body,
        &request.model,
        runtime_config::thinking_level().as_deref(),
    );
    body
}
