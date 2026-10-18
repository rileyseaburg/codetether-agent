//! Model picker catalog state operations.
use crate::ModelPickerState;
impl ModelPickerState {
    /// Returns catalog entries matching every query term.
    pub fn filtered_models(&self) -> Vec<&str> {
        crate::matching(&self.available_models, &self.model_filter)
    }

    /// Replaces the catalog and clamps the current selection.
    pub fn set_available_models(&mut self, models: Vec<String>) {
        self.available_models = models;
        if self.selected_model_index >= self.filtered_models().len() {
            self.selected_model_index = self.filtered_models().len().saturating_sub(1);
        }
    }

    /// Returns the selected model in the filtered catalog.
    pub fn selected_model(&self) -> Option<&str> {
        self.filtered_models()
            .get(self.selected_model_index)
            .copied()
    }
}
