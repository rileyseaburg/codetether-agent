#![cfg(feature = "tetherscript")]

use codetether_agent::provider::Provider;

#[tokio::test]
async fn cerebras_tetherscript_models_include_required_model_info_fields() {
    let provider =
        codetether_agent::provider::tetherscript_provider::cerebras::new("test-key", None)
            .expect("cerebras provider should initialize");

    let models = provider
        .list_models()
        .await
        .expect("cerebras list_models should deserialize into ModelInfo");

    assert!(!models.is_empty());
    assert!(models.iter().any(|model| model.id == "gpt-oss-120b"));

    for model in models {
        assert_eq!(model.provider, "cerebras");
        assert_eq!(model.context_window, 131_072);
        assert_eq!(model.max_output_tokens, Some(16_384));
        assert!(!model.supports_vision);
        assert!(model.supports_tools);
        assert!(model.supports_streaming);
    }
}
