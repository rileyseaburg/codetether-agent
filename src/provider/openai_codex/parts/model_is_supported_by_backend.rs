impl OpenAiCodexProvider {
    /// Whether `model` is known for the active backend.
    ///
    /// API-key mode accepts every model. On the ChatGPT backend a model is
    /// known when it appears in the latest live discovery or the offline seed.
    fn model_is_supported_by_backend(&self, model: &str) -> bool {
        if !self.using_chatgpt_backend() {
            return true;
        }
        model_discovery::discovered_slugs().is_some_and(|slugs| slugs.iter().any(|s| s == model))
            || Self::chatgpt_supported_models().contains(&model)
    }
}
