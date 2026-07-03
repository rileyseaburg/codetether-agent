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
    (!fields.is_empty()).then(|| Value::Object(fields))
}

fn configured_service_tier() -> Option<String> {
    super::super::runtime_config::service_tier()
}

fn configured_effort() -> &'static str {
    let raw = super::super::runtime_config::thinking_effort().unwrap_or_default();
    match raw.as_str() {
        "low" => "low",
        "high" => "high",
        _ => "medium",
    }
}

fn uses_adaptive_thinking(model_id: &str) -> bool {
    super::super::output_budget::has_encrypted_reasoning(model_id)
        || model_id.to_ascii_lowercase().contains("claude-mythos-5")
}

#[cfg(test)]
mod tests {
    use super::additional_model_request_fields;

    #[test]
    fn fable_fields_include_adaptive_thinking() {
        let fields = additional_model_request_fields("global.anthropic.claude-fable-5").unwrap();
        assert_eq!(fields["thinking"]["type"], "adaptive");
        assert_eq!(fields["output_config"]["effort"], "medium");
    }
}