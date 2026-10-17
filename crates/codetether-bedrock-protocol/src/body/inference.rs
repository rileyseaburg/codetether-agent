//! Serialize inference controls without sending unsupported model parameters.

use codetether_provider_core::CompletionRequest;
use serde_json::{Value, json};

pub(super) fn build(request: &CompletionRequest, model_id: &str) -> Value {
    let mut config = json!({
        "maxTokens": super::super::output_budget::effective_max_tokens(request.max_tokens, model_id)
    });
    if let Some(temperature) = request.temperature {
        if omits_temperature(model_id) {
            tracing::debug!(provider = "bedrock", model = %model_id,
                "Omitting unsupported temperature parameter");
        } else {
            config["temperature"] = json!(temperature);
        }
    }
    if let Some(top_p) = request.top_p {
        config["topP"] = json!(top_p);
    }
    config
}

/// Whether this model rejects an explicit temperature.
///
/// # Arguments
/// * `model_id` - Full, prefixed, or short model identifier.
///
/// # Returns
/// Whether request construction should omit temperature.
///
/// # Examples
/// ```
/// assert!(codetether_bedrock_protocol::body::inference::omits_temperature("openai.gpt-6-astra"));
/// ```
pub fn omits_temperature(model_id: &str) -> bool {
    if super::super::output_budget::has_encrypted_reasoning(model_id) {
        return true;
    }

    let id = model_id.to_ascii_lowercase();
    let id = id.rsplit('/').next().unwrap_or(&id);
    let id = ["us.", "eu.", "apac.", "global."]
        .iter()
        .find_map(|prefix| id.strip_prefix(prefix))
        .unwrap_or(id);
    let id = id.split([':', '@']).next().unwrap_or(id);
    let id = id.strip_prefix("openai.").unwrap_or(id);

    // Bedrock reports these as either "temperature is deprecated" or
    // "doesn't support the temperature field" depending on the model family.
    id.starts_with("anthropic.claude-haiku-5") || id.starts_with("gpt-6")
}

#[cfg(test)]
#[path = "inference_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "inference_sonnet5_tests.rs"]
mod sonnet5_tests;

#[cfg(test)]
#[path = "inference_haiku55_tests.rs"]
mod haiku55_tests;
