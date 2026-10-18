//! Verify discovered provider models in the picker catalog.
use super::*;
#[tokio::test]
async fn refresh_includes_non_codex_provider_models() {
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(StaticProvider {
        name: "openrouter",
        models: vec![model("openai/gpt-5.5", "openrouter")],
    }));
    let registry = Arc::new(registry);
    let mut state = ModelPickerState::default();

    let summary = state
        .refresh_available_models(Some(&registry))
        .await
        .expect("refresh should succeed");

    assert_eq!(state.available_models, ["openrouter/openai/gpt-5.5"]);
    assert_eq!(summary.loaded_models, 1);
    assert_eq!(summary.loaded_providers, 1);
}
