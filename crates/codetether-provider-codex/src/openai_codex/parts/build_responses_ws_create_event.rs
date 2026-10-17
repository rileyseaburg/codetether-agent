impl OpenAiCodexProvider {
    #[cfg(test)]
    fn build_responses_ws_create_event(
        request: &CompletionRequest,
        model: &str,
        reasoning_effort: Option<ThinkingLevel>,
        service_tier: Option<CodexServiceTier>,
    ) -> Value {
        Self::build_responses_ws_create_event_for_backend(
            request,
            model,
            reasoning_effort,
            service_tier,
            ResponsesWsBackend::OpenAi,
        )
    }
}
