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
    assert!(ids.contains(&"gpt-6-astra"));
    assert!(ids.contains(&"gpt-6-astra-fast"));
    assert!(ids.contains(&"gpt-reserve"));
    assert!(ids.contains(&"gpt-5.4"));
    assert!(ids.contains(&"gpt-5.4-mini"));
    assert!(ids.contains(&"codex-auto-review"));

    let astra = models.iter().find(|model| model.id == "gpt-6-astra").unwrap();
    assert_eq!(astra.context_window, 1_000_000);
    assert_eq!(astra.input_cost_per_million, Some(10.0));
    assert_eq!(astra.output_cost_per_million, Some(50.0));
    assert!(astra.supports_vision);
}