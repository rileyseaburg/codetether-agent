impl OpenAiCodexProvider {
    fn model_supports_reasoning_effort(model: &str) -> bool {
        let normalized = model.to_ascii_lowercase();
        normalized.starts_with("gpt-5")
            || normalized.starts_with("openai.gpt-5")
            || normalized.starts_with("o1")
            || normalized.starts_with("o3")
            || normalized.starts_with("o4")
    }
}
