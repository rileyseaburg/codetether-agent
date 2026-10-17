#[test]
fn rejects_pro_model_for_chatgpt_backend() {
    // Unknown models are no longer rejected locally: the Codex backend is
    // authoritative, so newly released models (e.g. gpt-6.1-sol) work
    // without a CodeTether release.
    let provider = OpenAiCodexProvider::new();
    assert!(provider.validate_model_for_backend("gpt-5.4-pro").is_ok());
    assert!(provider.validate_model_for_backend("gpt-6.1-sol").is_ok());
}
