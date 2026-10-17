//! Assemble shared buffered completion fields without choosing history policy.
use super::BufferedFormat;
use crate::CompletionRequest;
use serde_json::{Value, json};

/// Assemble a buffered request using the caller's encoded history and policies.
///
/// # Arguments
/// * `request` - Model and generation settings.
/// * `messages` - History encoded using the caller's existing conventions.
/// * `tools` - Encoded tool definitions.
/// * `thinking` - Caller-selected thinking configuration.
/// * `format` - Which optional fields the caller previously emitted.
///
/// # Returns
/// The buffered JSON payload without performing I/O.
///
/// # Examples
/// ```no_run
/// # fn example(request: &codetether_provider_zai::CompletionRequest) {
/// use codetether_provider_zai::request::{encode_buffered, BufferedFormat};
/// let body = encode_buffered(request, vec![], vec![], serde_json::json!({}), BufferedFormat::Application);
/// assert!(body.get("stream").is_none());
/// # }
/// ```
pub fn encode_buffered(
    request: &CompletionRequest,
    messages: Vec<Value>,
    tools: Vec<Value>,
    thinking: Value,
    format: BufferedFormat,
) -> Value {
    let mut body = json!({
        "model": request.model,
        "messages": messages,
        "temperature": request.temperature.unwrap_or(1.0),
    });
    body["thinking"] = thinking;
    if !tools.is_empty() {
        body["tools"] = json!(tools);
    }
    if let Some(max) = request.max_tokens {
        body["max_tokens"] = json!(max);
    }
    if let BufferedFormat::ExplicitControls = format {
        super::controls::apply(&mut body, request);
    }
    body
}
