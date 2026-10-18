//! `AppState` integration helpers for the local model store.

use super::AppState;

impl AppState {
    /// Populate `available_models` from the on-disk store when empty and
    /// return the resulting model count.
    ///
    /// Lets the picker render instantly while a fresh list loads in the
    /// background. No-op when models are already loaded or the store is empty.
    pub fn hydrate_models_from_store(&mut self) -> usize {
        self.model_picker.hydrate_models_from_store()
    }

    /// Persist the current `available_models` to the on-disk store.
    ///
    /// No-op when the list is empty or the store path cannot be resolved.
    pub fn persist_models_to_store(&self) {
        self.model_picker.persist_models_to_store();
    }

    /// Apply a freshly fetched model list, clear the in-flight refresh state,
    /// and persist the result to the on-disk store in one step.
    pub fn finish_model_refresh(&mut self, models: Vec<String>) {
        self.model_picker.finish_model_refresh(models);
    }
}
