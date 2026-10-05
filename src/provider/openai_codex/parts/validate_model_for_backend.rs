impl OpenAiCodexProvider {
    /// Check `model` against the known catalog without blocking the request.
    ///
    /// The Codex backend is authoritative for which models an account may
    /// use, so an unknown model (for example one released after this build)
    /// is sent upstream instead of being rejected locally. Unknown models are
    /// logged so a later upstream error is easy to attribute.
    fn validate_model_for_backend(&self, model: &str) -> Result<()> {
        let (resolved_model, _, _) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(model);
        if self.using_chatgpt_backend()
            && !self.model_is_supported_by_backend(model)
            && !self.model_is_supported_by_backend(&resolved_model)
        {
            tracing::info!(provider = "openai-codex", model = %model,
                "Model not in the known Codex catalog; letting the backend decide");
        }
        Ok(())
    }
}
