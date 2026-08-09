//! Build the JSON body for a Bedrock Converse API request.
//!
//! Translates a [`CompletionRequest`] plus a resolved model ID into the
//! exact JSON shape expected by the Bedrock runtime's `/converse` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::build_converse_body;
//! use codetether_agent::provider::CompletionRequest;
//!
//! let request = CompletionRequest {
//!     model: "claude-sonnet-4".to_string(),
//!     messages: vec![],
//!     tools: vec![],
//!     temperature: Some(0.5),
//!     top_p: None,
//!     max_tokens: Some(1024),
//!     stop: vec![],
//! };
//! let body = build_converse_body(&request, "us.anthropic.claude-sonnet-4-20250514-v1:0");
//! assert_eq!(body["inferenceConfig"]["maxTokens"], 1024);
//! assert_eq!(body["inferenceConfig"]["temperature"], 0.5);
//! ```

use super::convert::{convert_messages, convert_tools};
use super::output_budget::effective_max_tokens;
use fields::additional_model_request_fields;
use {crate::provider::CompletionRequest, serde_json::Value, serde_json::json};

pub(super) mod fields;

#[path = "audit.rs"]
pub(super) mod audit;
#[path = "body/cache.rs"]
mod cache;

#[cfg(test)]
#[path = "body/pairing_tests.rs"]
mod pairing_tests;

/// Build the JSON body for a Bedrock Converse API request.
///
/// # Arguments
///
/// * `request` — The generic completion request from the session layer.
/// * `model_id` — The already-resolved Bedrock model ID (as returned by
///   [`super::resolve_model_id`]). Used to decide model-specific quirks
///   such as omitting `temperature` for newer Claude reasoning models.
///
/// # Returns
///
/// A [`serde_json::Value`] ready to be serialized and POSTed to
/// `/model/{id}/converse`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::build_converse_body;
/// use codetether_agent::provider::CompletionRequest;
///
/// let req = CompletionRequest {
///     model: "claude-opus-4-7".into(),
///     messages: vec![],
///     tools: vec![],
///     temperature: Some(0.7),
///     top_p: None,
///     max_tokens: None,
///     stop: vec![],
/// };
/// let body = build_converse_body(&req, "us.anthropic.claude-opus-4-7");
/// // Opus 4.7 omits temperature
/// assert!(body["inferenceConfig"].get("temperature").is_none());
/// ```
pub fn build_converse_body(request: &CompletionRequest, model_id: &str) -> Value {
    let (mut system_parts, mut messages) = convert_messages(&request.messages);
    let mut tools = convert_tools(&request.tools);

    // Anthropic prompt caching on Bedrock uses `cachePoint` content blocks;
    // see [`cache`] for breakpoint placement and the opt-out env var.
    cache::apply(model_id, &mut system_parts, &mut tools, &mut messages);

    let mut body = json!({"messages": messages});

    if !system_parts.is_empty() {
        body["system"] = json!(system_parts);
    }

    let mut inference_config = json!({});
    inference_config["maxTokens"] = json!(effective_max_tokens(request.max_tokens, model_id));

    let skip_temperature = super::output_budget::has_encrypted_reasoning(model_id);
    if let Some(temp) = request.temperature {
        if !skip_temperature {
            inference_config["temperature"] = json!(temp);
        } else {
            tracing::debug!(
                provider = "bedrock",
                model = %model_id,
                "Skipping temperature parameter (deprecated for this model)"
            );
        }
    }
    if let Some(top_p) = request.top_p {
        inference_config["topP"] = json!(top_p);
    }
    body["inferenceConfig"] = inference_config;

    if let Some(fields) = additional_model_request_fields(model_id) {
        body["additionalModelRequestFields"] = fields;
    }

    if !tools.is_empty() {
        body["toolConfig"] = json!({"tools": tools});
    }

    // Final gate: never ship an assistant toolUse whose toolResult is
    // missing. Bedrock answers that with a permanent 400, killing the turn.
    audit::enforce(&mut body);

    body
}
