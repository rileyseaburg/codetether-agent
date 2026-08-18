//! Bedrock Converse request building for the thinker backend.

use super::ThinkerConfig;

/// Build the Converse request body for `config` and the given prompts.
pub(super) fn body(
    config: &ThinkerConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> serde_json::Value {
    let mut body = serde_json::json!({
        "system": [{"text": system_prompt}],
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
pub(super) fn url(config: &ThinkerConfig) -> String {
    format!(
        "https://bedrock-runtime.{}.amazonaws.com/model/{}/converse",
        config.bedrock_region, config.model
    )
}
