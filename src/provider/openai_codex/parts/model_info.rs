impl OpenAiCodexProvider {
    fn model_info(
        id: &str,
        name: &str,
        context_window: usize,
        max_output_tokens: usize,
        supports_vision: bool,
    ) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
            provider: "openai-codex".to_string(),
            context_window,
            max_output_tokens: Some(max_output_tokens),
            supports_vision,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(0.0),
            output_cost_per_million: Some(0.0),
        }
    }
}
