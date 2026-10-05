//! Serialize inference controls without sending unsupported model parameters.

use crate::provider::CompletionRequest;
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

fn omits_temperature(model_id: &str) -> bool {
    // Encrypted-reasoning models (incl. Sonnet 5.x) reject `temperature`.
    if super::super::output_budget::has_encrypted_reasoning(model_id) {
        return true;
    }
    let id = model_id.to_ascii_lowercase();
    let id = id.rsplit('/').next().unwrap_or(&id);
    let id = ["us.", "eu.", "apac.", "global."]
        .iter()
        .find_map(|prefix| id.strip_prefix(prefix))
        .unwrap_or(id);
    let id = id.strip_prefix("openai.").unwrap_or(id);
    let id = id.split([':', '@']).next().unwrap_or(id);
    // Bedrock rejects `temperature` for the whole GPT-6.x family
    // ("This model doesn't support the temperature field").
    id.starts_with("gpt-6")
}

#[cfg(test)]
#[path = "inference_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "inference_sonnet5_tests.rs"]
mod sonnet5_tests;
