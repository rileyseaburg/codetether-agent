//! Model picker navigation state operations.
use crate::ModelPickerState;
impl ModelPickerState {
    /// Moves selection up, saturating at zero.
    pub fn model_select_prev(&mut self) {
        self.selected_model_index = self.selected_model_index.saturating_sub(1);
    }

    /// Moves selection down while another filtered model exists.
    pub fn model_select_next(&mut self) {
        if self.selected_model_index + 1 < self.filtered_models().len() {
            self.selected_model_index += 1;
        }
    }

    /// Appends a query character and resets selection.
    pub fn model_filter_push(&mut self, c: char) {
        self.model_filter.push(c);
        self.selected_model_index = 0;
    }

    /// Removes the final query character and resets selection.
    pub fn model_filter_backspace(&mut self) {
        self.model_filter.pop();
        self.selected_model_index = 0;
    }
}
