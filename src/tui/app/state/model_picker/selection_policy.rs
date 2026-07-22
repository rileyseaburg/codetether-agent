//! Product policy for models exposed by the interactive selector.

use crate::provider::ModelInfo;

pub(super) fn model_ref(provider: &str, model: &ModelInfo) -> Option<String> {
    let provider = provider.trim();
    let model_id = model.id.trim();
    if provider.is_empty() || model_id.is_empty() {
        return None;
    }
    let prefix = format!("{provider}/");
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
    fn admits_models_from_every_provider() {
        assert_eq!(
            model_ref("google", &model("gemini-2.5-pro")).as_deref(),
            Some("google/gemini-2.5-pro")
        );
        assert_eq!(
            model_ref("openrouter", &model("openrouter/openai/gpt-5.5")).as_deref(),
            Some("openrouter/openai/gpt-5.5")
        );
        assert!(model_ref("", &model("gpt-5.6-sol")).is_none());
        assert!(model_ref("openai-codex", &model(" ")).is_none());
    }
}
