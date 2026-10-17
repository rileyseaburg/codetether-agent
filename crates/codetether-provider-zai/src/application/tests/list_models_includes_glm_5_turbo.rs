//! Migrated application provider regression.
use super::*;
#[tokio::test]
async fn list_models_includes_glm_5_turbo() {
    let provider = ZaiProvider::with_base_url("test-key".to_string(), DEFAULT_BASE_URL.to_string())
        .expect("provider should construct");
    let models = provider.list_models().await.expect("models should list");

    let turbo = models
        .iter()
        .find(|m| m.id == "glm-5-turbo")
        .expect("glm-5-turbo should be in model list");
    assert_eq!(turbo.context_window, 200_000);
    assert_eq!(turbo.max_output_tokens, Some(128_000));
    assert!(turbo.supports_tools);
    assert!(turbo.supports_streaming);
    assert_eq!(turbo.input_cost_per_million, Some(0.96));
    assert_eq!(turbo.output_cost_per_million, Some(3.20));
}
