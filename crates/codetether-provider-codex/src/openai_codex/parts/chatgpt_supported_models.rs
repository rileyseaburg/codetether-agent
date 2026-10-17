impl OpenAiCodexProvider {
    fn chatgpt_supported_models() -> &'static [&'static str] {
        model_catalog::chatgpt_models()
    }
}
