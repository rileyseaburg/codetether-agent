impl OpenAiCodexProvider {
    fn parse_model_thinking_level(model: &str) -> (String, Option<ThinkingLevel>) {
        let Some((base_model, level_str)) = model.rsplit_once(':') else {
            return (model.to_string(), None);
        };
        let Some(level) = ThinkingLevel::parse(level_str) else {
            return (model.to_string(), None);
        };
        if base_model.trim().is_empty() {
            return (model.to_string(), None);
        }
        (base_model.to_string(), Some(level))
    }
}
