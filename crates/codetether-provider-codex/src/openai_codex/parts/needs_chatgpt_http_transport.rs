impl OpenAiCodexProvider {
    fn needs_chatgpt_http_transport(model: &str) -> bool {
        let (model, _, _) = Self::resolve_model_and_reasoning_effort_and_service_tier(model);
        transport_catalog::requires_http(&model)
    }
}
