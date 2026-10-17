impl OpenAiCodexProvider {
    /// Whether `model` is known for the active backend.
    ///
    /// API-key mode accepts every model. On the ChatGPT backend a model is
    /// known when it appears in the latest live discovery or the offline seed.
    fn model_is_supported_by_backend(&self, model: &str) -> bool {
        codetether_provider_core::codex::model_policy::model_is_supported_by_backend(
            self.using_chatgpt_backend(),
            model,
        )
    }
}
