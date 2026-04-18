//! Async model list refresh from the provider registry.

use std::sync::Arc;

use crate::provider::ProviderRegistry;

impl super::AppState {
    /// Refresh the available models list from the provider registry.
    ///
    /// # Errors
    ///
    /// Returns an error if any provider call fails critically.
    pub async fn refresh_available_models(
        &mut self,
        registry: Option<&Arc<ProviderRegistry>>,
    ) -> anyhow::Result<()> {
        let mut models = Vec::new();
        let Some(registry) = registry else {
            self.available_models.clear();
            return Ok(());
        };

        for provider_name in registry.list() {
            if let Some(provider) = registry.get(provider_name)
                && let Ok(provider_models) = provider.list_models().await
            {
                models.extend(provider_models.into_iter().map(|model| {
                    if model.id.contains('/') {
                        model.id
                    } else {
                        format!("{provider_name}/{}", model.id)
                    }
                }));
            }
        }

        models.sort();
        models.dedup();
        self.set_available_models(models);
        Ok(())
    }
}
