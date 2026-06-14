//! Request-body construction for Anthropic Messages API calls.
//!
//! This module is responsible for assembling the JSON payload sent to the
//! Anthropic Messages API. It combines the provider-neutral
//! [`CompletionRequest`] fields with Anthropic-specific message and tool
//! conversion helpers.
//!
//! The builder keeps request-shaping logic separate from HTTP execution. It
//! does not send network requests, mutate provider state, or validate API
//! credentials; callers are responsible for those concerns before submitting
//! the returned JSON body.

use serde_json::{Value, json};

use crate::provider::CompletionRequest;

/// Build the JSON body for a completion request.
///
/// The returned [`Value`] follows the Anthropic Messages API schema:
/// it always includes the target model, converted message list, and
/// `max_tokens`; it conditionally includes system blocks, tool definitions,
/// temperature, and top-p sampling when those values are present on the
/// request.
///
/// `enable_cache` controls whether the message and tool conversion helpers add
/// Anthropic ephemeral prompt-cache markers to eligible blocks.
///
/// # Arguments
///
/// * `req` - Provider-neutral completion request containing model, messages,
///   tools, token limit, and optional sampling parameters.
/// * `enable_cache` - Whether converted system/message/tool blocks should be
///   annotated for Anthropic prompt caching.
///
/// # Returns
///
/// A JSON object ready to serialize as the request body for
/// `POST /v1/messages`.
///
/// # Side Effects
///
/// This function has no side effects. It only reads from `req` and allocates a
/// new JSON value.
///
/// # Preconditions
///
/// The request should already contain messages in the provider-neutral format
/// expected by the conversion layer. API-key validation and HTTP-specific
/// header construction are handled by the caller.
pub(crate) fn build(req: &CompletionRequest, enable_cache: bool) -> Value {
    let mut body = build_base(req, enable_cache);
    if is_minimax_m3(&req.model) {
        body["thinking"] = json!({"type": "adaptive"});
    }
    body
}

/// Build the JSON body for a streaming completion request.
///
/// Identical to [`build`] except `"stream": true` is included.
pub(crate) fn build_streaming(req: &CompletionRequest, enable_cache: bool) -> Value {
    let mut body = build_base(req, enable_cache);
    body["stream"] = json!(true);
    if is_minimax_m3(&req.model) {
        body["thinking"] = json!({"type": "adaptive"});
    }
    body
}

/// Assemble the common JSON body shared by streaming and non-streaming paths.
fn build_base(req: &CompletionRequest, enable_cache: bool) -> Value {
    let (system_prompt, messages) = super::convert::messages(&req.messages, enable_cache);
    let tools = super::convert_tools::tools(&req.tools, enable_cache);
    let mut body = json!({
        "model": req.model,
        "messages": messages,
        "max_tokens": req.max_tokens.unwrap_or(8192),
    });
    if let Some(system) = system_prompt {
        body["system"] = json!(system);
    }
    if !tools.is_empty() {
        body["tools"] = json!(tools);
    }
    if let Some(temp) = req.temperature {
        body["temperature"] = json!(temp);
    }
    if let Some(top_p) = req.top_p {
        body["top_p"] = json!(top_p);
    }
    body
}

/// Check whether the model is MiniMax M3 (supports thinking control).
fn is_minimax_m3(model: &str) -> bool {
    model.eq_ignore_ascii_case("MiniMax-M3")
}
