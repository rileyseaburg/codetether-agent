//! Product policy for models exposed by the interactive selector.

use crate::provider::ModelInfo;

pub(super) fn model_ref(provider: &str, model: &ModelInfo) -> Option<String> {
    let provider = provider.trim();
    let model_id = model.id.trim();
    let prefix = format!("{provider}/");
    let unqualified = model_id.strip_prefix(&prefix).unwrap_or(model_id);
    if provider != "openai-codex" || !unqualified.starts_with("gpt-5.6-") {
        return None;
    }
    Some(if model_id.starts_with(&prefix) {
        model_id.to_string()
    } else {
        format!("{provider}/{model_id}")
    })
}

#[cfg(test)]
mod tests {
    use super::model_ref;
    use crate::provider::ModelInfo;

    fn model(id: &str) -> ModelInfo {
        ModelInfo {
            id: id.into(),
            name: id.into(),
            provider: "openai-codex".into(),
            context_window: 272_000,
            max_output_tokens: Some(128_000),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: None,
            output_cost_per_million: None,
        }
    }

    #[test]
    fn admits_only_openai_codex_5_6_models() {
        assert_eq!(
            model_ref("openai-codex", &model("gpt-5.6-sol")).as_deref(),
            Some("openai-codex/gpt-5.6-sol")
        );
        assert!(model_ref("openai-codex", &model("gpt-5.5")).is_none());
        assert!(model_ref("bedrock", &model("gpt-5.6-sol")).is_none());
        assert!(model_ref("openai-codex", &model("openai-codex/gpt-5.6-sol")).is_some());
    }
}
