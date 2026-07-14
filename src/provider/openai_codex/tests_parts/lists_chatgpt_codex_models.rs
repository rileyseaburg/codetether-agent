#[tokio::test]
async fn lists_chatgpt_codex_models() {
    let provider = OpenAiCodexProvider::new();
    let models = provider
        .list_models()
        .await
        .expect("model listing should succeed");

    let ids = models
        .iter()
        .map(|model| model.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, OpenAiCodexProvider::chatgpt_supported_models());
}
