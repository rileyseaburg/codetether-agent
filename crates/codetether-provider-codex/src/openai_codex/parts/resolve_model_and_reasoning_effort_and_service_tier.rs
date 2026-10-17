impl OpenAiCodexProvider {
    /// Returns the upstream model without exposing private request options.
    pub fn resolved_model_identity(model: &str) -> String {
        Self::resolve_model_and_reasoning_effort_and_service_tier(model).0
    }

    fn resolve_model_and_reasoning_effort_and_service_tier(
        model: &str,
    ) -> (String, Option<ThinkingLevel>, Option<CodexServiceTier>) {
        codetether_provider_core::codex::model_policy::resolve_model_and_reasoning_effort_and_service_tier(model, Self::configured_thinking_level)
    }
}
