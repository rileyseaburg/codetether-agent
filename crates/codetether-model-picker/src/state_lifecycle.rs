//! Model picker lifecycle state operations.
use crate::ModelPickerState;
impl ModelPickerState {
    /// Opens the picker and clears the filter and selection.
    pub fn open_model_picker(&mut self) {
        self.model_picker_active = true;
        self.model_filter.clear();
        self.selected_model_index = 0;
    }

    /// Closes the picker and clears verifier mode, filter and selection.
    pub fn close_model_picker(&mut self) {
        self.model_picker_active = false;
        self.model_picker_for_verifier = false;
        self.model_filter.clear();
        self.selected_model_index = 0;
    }
}
