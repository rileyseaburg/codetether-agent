//! Application model-refresh tasks and state updates.
pub use codetether_model_discovery::{ModelRefreshEvent, ModelRefreshSummary};
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::models::ViewMode;

impl super::AppState {
    pub fn start_model_refresh(&mut self, registry: Arc<ProviderRegistry>) {
        self.model_picker.start_model_refresh(registry);
    }
    pub fn drain_model_refresh(&mut self) {
        if let Some(status) = self
            .model_picker
            .drain_refresh_status(self.view_mode == ViewMode::Model)
        {
            self.status = status;
        }
    }

    /// Refresh the available models list from the provider registry.
    ///
    /// # Errors
    ///
    /// Returns an error if any provider call fails critically.
    pub async fn refresh_available_models(
        &mut self,
        registry: Option<&Arc<ProviderRegistry>>,
    ) -> anyhow::Result<ModelRefreshSummary> {
        self.model_picker.refresh_available_models(registry).await
    }
}
