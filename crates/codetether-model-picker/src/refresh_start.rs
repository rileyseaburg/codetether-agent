//! Background model discovery task and receiver installation.
use crate::ModelPickerState;
use codetether_model_discovery::{ModelRefreshEvent, Registry, load_available_models};
use std::sync::Arc;
use tokio::sync::mpsc;
impl ModelPickerState {
    /// Starts discovery with the supplied registry and installs a completion receiver.
    ///
    /// # Panics
    /// Panics when called without an active Tokio runtime.
    /// # Examples
    /// ```no_run
    /// # fn example<R: codetether_model_discovery::Registry + Send + 'static>(registry: std::sync::Arc<R>) {
    /// let mut state = codetether_model_picker::ModelPickerState::default();
    /// state.start_model_refresh(registry);
    /// # }
    /// ```
    pub fn start_model_refresh<R: Registry + Send + 'static>(&mut self, registry: Arc<R>) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.model_refresh_rx = Some(rx);
        self.model_refresh_in_flight = true;

        tokio::spawn(async move {
            let (models, summary) = load_available_models(registry.as_ref()).await;
            let _ = tx.send(ModelRefreshEvent::Loaded { models, summary });
        });
    }
}
