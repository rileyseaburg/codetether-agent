#[tokio::test]
async fn omits_gpt_5_5_from_api_key_model_listing_for_now() {
    let provider = OpenAiCodexProvider::from_api_key("test-key".to_string());
    let models = provider
        .list_models()
        .await
        .expect("model listing should succeed");

    assert!(!models.iter().any(|model| model.id == "gpt-5.5"));
    assert!(!models.iter().any(|model| model.id == "gpt-5.5-fast"));
    assert!(!models.iter().any(|model| model.id.starts_with("gpt-5.6")));
}
