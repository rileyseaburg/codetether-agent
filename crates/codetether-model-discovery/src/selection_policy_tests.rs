//! Model reference policy regression test.

use super::model_ref;
use codetether_provider_core::ModelInfo;

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
