//! Unit tests for model resolution
//! Validates that gpt-5-codex variants are properly parsed

#[cfg(test)]
mod tests {
    use codetether_agent::provider::parse_model_string;

    #[test]
    fn test_parse_model_string_bare() {
        let (provider, model) = parse_model_string("gpt-5-codex");
        assert_eq!(provider, None);
        assert_eq!(model, "gpt-5-codex");
    }

    #[test]
    fn test_parse_model_string_with_provider() {
        let (provider, model) = parse_model_string("openai/gpt-5-codex");
        assert_eq!(provider, Some("openai"));
        assert_eq!(model, "gpt-5-codex");
    }

    #[test]
    fn test_parse_model_string_openrouter_format() {
        let (provider, model) = parse_model_string("openrouter/openai/gpt-5-codex");
        assert_eq!(provider, Some("openrouter"));
        assert_eq!(model, "openai/gpt-5-codex");
    }

    #[test]
    fn test_parse_model_string_with_version() {
        for model_id in &[
            "gpt-5-codex",
            "gpt-5.1-codex",
            "gpt-5.2-codex",
            "gpt-5.3-codex",
        ] {
            let (provider, model) = parse_model_string(model_id);
            assert_eq!(provider, None);
            assert_eq!(model, *model_id);
        }
    }

    #[test]
    fn test_parse_model_string_copilot_prefixed() {
        let (provider, model) = parse_model_string("copilot/gpt-5-codex");
        assert_eq!(provider, Some("copilot"));
        assert_eq!(model, "gpt-5-codex");
    }
}
