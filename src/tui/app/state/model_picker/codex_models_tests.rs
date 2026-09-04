// Pre-5.6 Codex models remain selectable with fast and reasoning variants.
#[tokio::test]
async fn refresh_includes_pre_5_6_codex_models() {
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(StaticProvider {
        name: "openai-codex",
        models: vec![model("gpt-5.5", "openai-codex")],
    }));
    let registry = Arc::new(registry);
    let mut state = super::super::AppState::default();

    let summary = state
        .refresh_available_models(Some(&registry))
        .await
        .expect("refresh should succeed");

    assert_eq!(
        state.available_models,
        [
            "openai-codex/gpt-5.5",
            "openai-codex/gpt-5.5-fast",
            "openai-codex/gpt-5.5-fast:high",
            "openai-codex/gpt-5.5-fast:low",
            "openai-codex/gpt-5.5-fast:medium",
            "openai-codex/gpt-5.5-fast:xhigh",
            "openai-codex/gpt-5.5:high",
            "openai-codex/gpt-5.5:low",
            "openai-codex/gpt-5.5:medium",
            "openai-codex/gpt-5.5:xhigh",
        ]
    );
    assert_eq!(summary.loaded_models, 10);
    assert_eq!(summary.loaded_providers, 1);
}
