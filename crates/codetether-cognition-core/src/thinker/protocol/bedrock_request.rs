//! Bedrock Converse request building for the thinker backend.

use super::ThinkerConfig;

/// Build the Converse request body for `config` and the given prompts.
///
/// # Arguments
/// * `config` - Thinker settings for the request or fallback model.
/// * `system_prompt` - Caller-owned system instructions.
/// * `user_prompt` - Caller-owned user text.
///
/// # Returns
/// The constructed request or normalized thinker output.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(config: &ThinkerConfig, system_prompt: &str, user_prompt: &str) {
/// let _result = codetether_cognition_core::thinker::protocol::bedrock_request::body(config, system_prompt, user_prompt);
/// # }
/// ```
pub fn body(config: &ThinkerConfig, system_prompt: &str, user_prompt: &str) -> serde_json::Value {
    let mut body = serde_json::json!({
        "system": [
            {"text": codetether_provider_core::request_identity::prompt("bedrock", &config.model)},
            {"text": codetether_provider_core::request_identity::caller_prompt(system_prompt)}
        ],
        "messages": [{
            "role": "user",
            "content": [{"text": user_prompt}]
        }],
        "inferenceConfig": {
            "maxTokens": config.max_tokens,
            "temperature": config.temperature
        }
    });

    if let Some(service_tier) = config.bedrock_service_tier.as_ref() {
        body["additionalModelRequestFields"] = serde_json::json!({
            "service_tier": service_tier
        });
    }
    body
}

/// Build the regional Converse endpoint URL.
///
/// Do NOT percent-encode `:` here — reqwest encodes URL paths natively, and
/// pre-encoding to `%3A` triggers double-encoding to `%253A`, breaking SigV4.
///
/// # Arguments
/// * `config` - Thinker settings for the request or fallback model.
///
/// # Returns
/// The regional Converse endpoint with the original model path.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(config: &ThinkerConfig) {
/// let _result = codetether_cognition_core::thinker::protocol::bedrock_request::url(config);
/// # }
/// ```
pub fn url(config: &ThinkerConfig) -> String {
    format!(
        "https://bedrock-runtime.{}.amazonaws.com/model/{}/converse",
        config.bedrock_region, config.model
    )
}
