impl OpenAiCodexProvider {
    #[cfg(test)]
    fn resolve_model_and_reasoning_effort(model: &str) -> (String, Option<ThinkingLevel>) {
        let (base_model, level, _) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(model);
        (base_model, level)
    }
}
