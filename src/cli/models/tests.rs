use super::{enrich, render, types::ProviderCapability};
use crate::provider::ModelInfo;

fn luna() -> ModelInfo {
    ModelInfo {
        id: "gpt-5.6-luna".into(),
        name: "Luna".into(),
        provider: "openai-codex".into(),
        context_window: 200_000,
        max_output_tokens: Some(32_000),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.0),
        output_cost_per_million: Some(0.0),
    }
}

#[test]
fn reports_luna_aliases_and_qualifiers() {
    let model = enrich::capability(luna());
    assert_eq!(model.canonical_id, "gpt-5.6-luna");
    assert_eq!(model.selectable_id, "openai-codex/gpt-5.6-luna");
    assert!(
        model
            .aliases
            .contains(&"openai-codex/gpt-5.6-luna-fast".into())
    );
    assert!(model.qualifiers.contains(&":high".into()));
    assert!(model.available);
    assert_eq!(model.source, "provider.list_models");
}

#[test]
fn renders_provider_discovery_failure() {
    let provider = ProviderCapability {
        provider: "openai-codex".into(),
        available: false,
        source: "configured provider registry",
        error: Some("offline".into()),
        models: vec![],
    };
    let text = render::text(&[provider]);
    assert!(text.contains("openai-codex [unavailable]"));
    assert!(text.contains("error: offline"));
}
