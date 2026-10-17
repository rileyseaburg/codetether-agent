//! Bedrock Converse response decoding for the thinker backend.

use super::ThinkerOutput;

/// Decode a successful Converse response body into a [`ThinkerOutput`].
///
/// # Arguments
/// * `model_id` - Resolved Bedrock model identifier.
/// * `parsed` - Decoded Bedrock JSON response.
///
/// # Returns
/// The constructed request or normalized thinker output.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(model_id: &str, parsed: &serde_json::Value) {
/// let _result = codetether_cognition_core::thinker::protocol::bedrock_response::decode(model_id, parsed);
/// # }
/// ```
pub fn decode(model_id: &str, parsed: &serde_json::Value) -> ThinkerOutput {
    let text = parsed["output"]["message"]["content"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|c| c["text"].as_str())
        .unwrap_or_default()
        .to_string();

    let usage = &parsed["usage"];
    let prompt_tokens = usage["inputTokens"].as_u64().map(|v| v as u32);
    let completion_tokens = usage["outputTokens"].as_u64().map(|v| v as u32);

    ThinkerOutput {
        model: model_id.to_string(),
        finish_reason: parsed["stopReason"].as_str().map(ToString::to_string),
        text,
        prompt_tokens,
        completion_tokens,
        total_tokens: prompt_tokens.zip(completion_tokens).map(|(p, c)| p + c),
        cache_read_tokens: None,
        cache_write_tokens: None,
    }
}
