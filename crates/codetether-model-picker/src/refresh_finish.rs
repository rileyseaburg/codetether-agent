//! Apply and persist a completed model catalog.
use crate::ModelPickerState;
impl ModelPickerState {
    /// Apply a freshly fetched model list, clear the in-flight refresh state,
    /// and persist the result to the on-disk store in one step.
    pub fn finish_model_refresh(&mut self, models: Vec<String>) {
        self.set_available_models(models);
        self.model_refresh_in_flight = false;
        self.model_refresh_rx = None;
        self.persist_models_to_store();
    }
}
