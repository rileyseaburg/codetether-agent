impl OpenAiCodexProvider {
    fn resolve_model_and_reasoning_effort_and_service_tier(
        model: &str,
    ) -> (String, Option<ThinkingLevel>, Option<CodexServiceTier>) {
        let (base_model, level_from_model) = Self::parse_model_thinking_level(model);
        let level = level_from_model.or_else(Self::configured_thinking_level);
        let (base_model, service_tier) = Self::parse_service_tier_model_alias(&base_model);
        if !Self::model_supports_reasoning_effort(&base_model) {
            return (base_model, None, service_tier);
        }
        let supported = reasoning_catalog::supported_levels(&base_model);
        let level = level.filter(|effort| {
            supported.is_empty() || reasoning_catalog::supports(&base_model, effort.as_str())
        });
        (base_model, level, service_tier)
    }
}
