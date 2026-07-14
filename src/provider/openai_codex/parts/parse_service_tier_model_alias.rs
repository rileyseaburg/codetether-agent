impl OpenAiCodexProvider {
    fn parse_service_tier_model_alias(model: &str) -> (String, Option<CodexServiceTier>) {
        if let Some(base) = service_tier_catalog::parse_fast_alias(model) {
            return (base.to_string(), Some(CodexServiceTier::Priority));
        }
        (model.to_string(), None)
    }
}
