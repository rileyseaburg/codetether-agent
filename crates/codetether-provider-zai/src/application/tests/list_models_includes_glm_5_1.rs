//! Migrated application provider regression.
use super::*;
#[tokio::test]
async fn list_models_includes_glm_5_1() {
    let provider = ZaiProvider::with_base_url("test-key".to_string(), DEFAULT_BASE_URL.to_string())
        .expect("provider should construct");
    let models = provider.list_models().await.expect("models should list");

    let glm51 = models
        .iter()
        .find(|m| m.id == "glm-5.1")
        .expect("glm-5.1 should be in model list");
    assert_eq!(glm51.context_window, 200_000);
    assert_eq!(glm51.max_output_tokens, Some(128_000));
    assert!(glm51.supports_tools);
    assert!(glm51.supports_streaming);
}
