//! Deterministic default-provider selection for AI session titles.

use anyhow::Result;

use crate::provider::ProviderRegistry;

use super::super::types::Session;

impl Session {
    /// Generate an AI title with the preferred configured provider.
    ///
    /// Falls back to the first-message heuristic when no provider is usable.
    ///
    /// # Errors
    ///
    /// Returns an error if heuristic title generation fails.
    pub async fn generate_ai_title(&mut self, registry: &ProviderRegistry) -> Result<()> {
        let providers = registry.list();
        let Some(name) = crate::session::helper::provider::choose_default_provider(&providers)
        else {
            return self.generate_title().await;
        };
        let Some(provider) = registry.get(name) else {
            return self.generate_title().await;
        };
        let model = crate::session::helper::defaults::default_model_for_provider(name);
        self.generate_ai_title_with(provider.as_ref(), &model).await
    }
}
