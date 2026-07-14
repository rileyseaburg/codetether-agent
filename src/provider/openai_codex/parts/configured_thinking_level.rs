impl OpenAiCodexProvider {
    fn configured_thinking_level() -> Option<ThinkingLevel> {
        runtime_config::thinking_level()
            .as_deref()
            .and_then(ThinkingLevel::parse)
    }
}
