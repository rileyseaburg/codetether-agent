use serde_json::{Map, Value, json};

pub(in crate::provider::bedrock) fn additional_model_request_fields(
    model_id: &str,
) -> Option<Value> {
    let mut fields = Map::new();
    if let Some(service_tier) = configured_service_tier() {
        tracing::debug!(
            provider = "bedrock",
            service_tier = %service_tier,
            "Applying Bedrock service tier override"
        );
        fields.insert("service_tier".into(), json!(service_tier));
    }
    if uses_adaptive_thinking(model_id) {
        fields.insert("thinking".into(), json!({"type": "adaptive"}));
        fields.insert(
            "output_config".into(),
            json!({"effort": configured_effort()}),
        );
    }
    if is_bedrock_openai_gpt(model_id) {
        // GPT-5.6 Sol/Terra/Luna (Codex rust-v0.143.0) support `max` effort.
        fields.insert("reasoning_effort".into(), json!(configured_effort()));
    }
    (!fields.is_empty()).then(|| Value::Object(fields))
}

fn configured_service_tier() -> Option<String> {
    super::super::runtime_config::service_tier()
}

pub(super) fn configured_effort() -> &'static str {
    let raw = super::super::runtime_config::thinking_effort().unwrap_or_default();
    match raw.as_str() {
        "low" => "low",
        "high" => "high",
        // `max` reasoning effort (GPT-5.6 Sol/Terra/Luna, Codex rust-v0.143.0)
        "max" => "max",
        _ => "medium",
    }
}

pub(super) fn uses_adaptive_thinking(model_id: &str) -> bool {
    super::super::output_budget::has_encrypted_reasoning(model_id)
        || model_id.to_ascii_lowercase().contains("claude-mythos-5")
}

/// Bedrock-hosted OpenAI GPT models (`openai.gpt-5.4/5.5/5.6-sol/terra/luna`).
pub(super) fn is_bedrock_openai_gpt(model_id: &str) -> bool {
    model_id.to_ascii_lowercase().contains("openai.gpt-5")
}

#[cfg(test)]
mod tests;
