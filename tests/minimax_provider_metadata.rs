use codetether_agent::provider::Provider;
use codetether_agent::provider::anthropic::AnthropicProvider;

#[tokio::test]
async fn minimax_models_include_m3_metadata() {
    let provider =
        AnthropicProvider::with_base_url("test-key".into(), "https://x".into(), "minimax")
            .expect("provider should construct");
    let models = provider.list_models().await.expect("models should list");
    let m3 = models.iter().find(|m| m.id == "MiniMax-M3").unwrap();

    assert_eq!(
        (m3.context_window, m3.max_output_tokens),
        (1_000_000, Some(128_000))
    );
    assert!(m3.supports_vision && m3.supports_tools && m3.supports_streaming);
    assert_eq!(
        (m3.input_cost_per_million, m3.output_cost_per_million),
        (Some(0.3), Some(1.2))
    );
}
