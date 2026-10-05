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
        let window = model
            .context_window
            .and_then(|w| usize::try_from(w).ok())
            .unwrap_or(272_000);
        let name = model
            .display_name
            .clone()
            .unwrap_or_else(|| model.slug.clone());
        Self::model_info(&model.slug, &name, window, 128_000, false)
    }
}
