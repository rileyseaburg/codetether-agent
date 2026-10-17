impl OpenAiCodexProvider {
    fn build_http_responses_body(request: &CompletionRequest) -> Value {
        let (model, reasoning, service_tier) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(&request.model);
        codetether_codex_protocol::CodexProtocol::build_http_responses_body(
            request,
            model,
            reasoning,
            service_tier,
        )
    }
}
