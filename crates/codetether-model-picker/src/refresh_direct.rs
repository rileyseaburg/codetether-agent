//! Direct model discovery for callers awaiting a catalog.
use crate::ModelPickerState;
use codetether_model_discovery::{ModelRefreshSummary, Registry, load_available_models};
use std::sync::Arc;
impl ModelPickerState {
    /// Refresh the available models list from the provider registry.
    ///
    /// # Errors
    ///
    /// Provider failures are counted in the summary and do not produce `Err`.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example<R: codetether_model_discovery::Registry>(registry: std::sync::Arc<R>) {
    /// let mut state = codetether_model_picker::ModelPickerState::default();
    /// let summary = state.refresh_available_models(Some(&registry)).await.unwrap();
    /// assert_eq!(summary.loaded_models, state.available_models.len());
    /// # }
    /// ```
    pub async fn refresh_available_models<R: Registry + ?Sized>(
        &mut self,
        registry: Option<&Arc<R>>,
    ) -> anyhow::Result<ModelRefreshSummary> {
        let Some(registry) = registry else {
            self.available_models.clear();
            return Ok(ModelRefreshSummary::empty());
        };

        let (models, summary) = load_available_models(registry.as_ref()).await;
        self.set_available_models(models);
        Ok(summary)
    }
}
