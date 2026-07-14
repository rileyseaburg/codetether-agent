impl OpenAiCodexProvider {
    fn api_key_hidden_model(model: &str) -> bool {
        matches!(
            model,
            "gpt-5.5" | "gpt-5.5-fast" | "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna"
        )
    }
}
