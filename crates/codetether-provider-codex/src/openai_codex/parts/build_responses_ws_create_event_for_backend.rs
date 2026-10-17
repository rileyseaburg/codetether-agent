impl OpenAiCodexProvider {
    fn build_responses_ws_create_event_for_backend(
        request: &CompletionRequest,
        model: &str,
        reasoning_effort: Option<ThinkingLevel>,
        service_tier: Option<CodexServiceTier>,
        backend: ResponsesWsBackend,
    ) -> Value {
        codetether_codex_protocol::CodexProtocol::build_responses_ws_create_event_for_backend(
            request,
            model,
            reasoning_effort,
            service_tier,
            backend,
        )
    }
}
