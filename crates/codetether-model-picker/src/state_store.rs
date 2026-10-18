//! Hydrate and persist the owned picker catalog.
use crate::ModelPickerState;
use codetether_model_store::ModelStore;
impl ModelPickerState {
    /// Populate `available_models` from the on-disk store when empty and
    /// return the resulting model count.
    ///
    /// Lets the picker render instantly while a fresh list loads in the
    /// background. No-op when models are already loaded or the store is empty.
    pub fn hydrate_models_from_store(&mut self) -> usize {
        if self.available_models.is_empty()
            && let Some(store) = ModelStore::open_default()
        {
            let cached = store.load();
            if !cached.is_empty() {
                self.set_available_models(cached);
            }
        }
        self.available_models.len()
    }

    /// Persist the current `available_models` to the on-disk store.
    ///
    /// No-op when the list is empty or the store path cannot be resolved.
    pub fn persist_models_to_store(&self) {
        if self.available_models.is_empty() {
            return;
        }
        if let Some(store) = ModelStore::open_default() {
            store.store(&self.available_models);
        }
    }
}
