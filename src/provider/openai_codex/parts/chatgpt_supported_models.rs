impl OpenAiCodexProvider {
    fn chatgpt_supported_models() -> &'static [&'static str] {
        transport_catalog::chatgpt_models()
    }
}
