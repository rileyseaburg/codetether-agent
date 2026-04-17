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
use crate::provider::CompletionRequest;
use serde_json::{Value, json};

/// Build the JSON body for a Bedrock Converse API request.
///
/// # Arguments
///
/// * `request` — The generic completion request from the session layer.
/// * `model_id` — The already-resolved Bedrock model ID (as returned by
///   [`super::resolve_model_id`]). Used to decide model-specific quirks
///   such as omitting `temperature` for Claude Opus 4.7.
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
/// let body = build_converse_body(&req, "us.anthropic.claude-opus-4-7-v1:0");
/// // Opus 4.7 omits temperature
/// assert!(body["inferenceConfig"].get("temperature").is_none());
/// ```
pub fn build_converse_body(request: &CompletionRequest, model_id: &str) -> Value {
    let (mut system_parts, messages) = convert_messages(&request.messages);
    let tools = convert_tools(&request.tools);

    // Anthropic prompt caching: mark the system prompt with a cache point so
    // subsequent requests with identical system text get a 90% input discount.
    // Only meaningful when CODETETHER_BEDROCK_PROMPT_CACHE is not "0"/"false".
    if prompt_cache_enabled()
        && supports_prompt_caching(model_id)
        && !system_parts.is_empty()
    {
        system_parts.push(json!({"cachePoint": {"type": "default"}}));
    }

    let mut body = json!({"messages": messages});

    if !system_parts.is_empty() {
        body["system"] = json!(system_parts);
    }

    let mut inference_config = json!({});
    inference_config["maxTokens"] = json!(request.max_tokens.unwrap_or(8192));

    let skip_temperature = model_id.to_ascii_lowercase().contains("claude-opus-4-7");
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

    if let Some(service_tier) = configured_service_tier() {
        tracing::debug!(
            provider = "bedrock",
            service_tier = %service_tier,
            "Applying Bedrock service tier override"
        );
        body["additionalModelRequestFields"] = json!({"service_tier": service_tier});
    }

    if !tools.is_empty() {
        body["toolConfig"] = json!({"tools": tools});
    }

    body
}

/// Read the `CODETETHER_BEDROCK_SERVICE_TIER` env var and return it normalized
/// (lowercased, trimmed). Returns `None` when unset or empty.
fn configured_service_tier() -> Option<String> {
    std::env::var("CODETETHER_BEDROCK_SERVICE_TIER")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .filter(|v| !v.is_empty())
}

/// Prompt caching is on by default; set `CODETETHER_BEDROCK_PROMPT_CACHE=0`
/// to disable.
fn prompt_cache_enabled() -> bool {
    match std::env::var("CODETETHER_BEDROCK_PROMPT_CACHE") {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off"),
        Err(_) => true,
    }
}

/// Returns true for model families that honor the `cachePoint` content block.
/// Currently: Anthropic Claude (3.5+, 4.x).
fn supports_prompt_caching(model_id: &str) -> bool {
    let id = model_id.to_ascii_lowercase();
    id.contains("anthropic") || id.contains("claude")
}
