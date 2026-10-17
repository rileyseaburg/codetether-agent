impl OpenAiCodexProvider {
    /// Models offered by the provider, preferring live discovery.
    ///
    /// On the ChatGPT backend the account's `/models` catalog is
    /// authoritative; the static seed list is used only when discovery fails
    /// (offline, logged out, or backend error), and that failure is logged.
    async fn listed_models(&self) -> Vec<ModelInfo> {
        if !self.using_chatgpt_backend() {
            return self.available_models();
        }
        match self.discover_chatgpt_models().await {
            Ok(models) => {
                let listed: Vec<ModelInfo> = models
                    .iter()
                    .filter(|model| model.is_listed())
                    .map(Self::discovered_model_info)
                    .collect();
                if listed.is_empty() {
                    self.available_models()
                } else {
                    listed
                }
            }
            Err(error) => {
                tracing::warn!(provider = "openai-codex", error = %error,
                    "Codex model discovery failed; using offline seed catalog");
                self.available_models()
            }
        }
    }

    fn discovered_model_info(model: &model_discovery::DiscoveredModel) -> ModelInfo {
        codetether_provider_core::codex::model_policy::discovered_model_info(model)
    }
}
