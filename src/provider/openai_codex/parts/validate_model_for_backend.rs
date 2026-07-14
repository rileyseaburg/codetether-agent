impl OpenAiCodexProvider {
    fn validate_model_for_backend(&self, model: &str) -> Result<()> {
        let (resolved_model, _, _) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(model);
        if self.model_is_supported_by_backend(model)
            || self.model_is_supported_by_backend(&resolved_model)
        {
            return Ok(());
        }

        if self.using_chatgpt_backend() {
            anyhow::bail!(
                "Model '{}' is not supported when using Codex with a ChatGPT account. Supported models: {}",
                model,
                Self::chatgpt_supported_models().join(", ")
            );
        }

        Ok(())
    }
}
