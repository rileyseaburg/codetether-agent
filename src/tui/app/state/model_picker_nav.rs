//! Compatibility methods forwarding to owned picker state.
impl super::AppState {
    pub fn filtered_models(&self) -> Vec<&str> {
        self.model_picker.filtered_models()
    }

    pub fn set_available_models(&mut self, models: Vec<String>) {
        self.model_picker.set_available_models(models);
    }

    pub fn open_model_picker(&mut self) {
        self.model_picker.open_model_picker();
        self.status = "Model picker".to_string();
    }

    pub fn close_model_picker(&mut self) {
        self.model_picker.close_model_picker();
    }

    pub fn model_select_prev(&mut self) {
        self.model_picker.model_select_prev();
    }

    pub fn model_select_next(&mut self) {
        self.model_picker.model_select_next();
    }

    pub fn model_filter_push(&mut self, c: char) {
        self.model_picker.model_filter_push(c);
    }

    pub fn model_filter_backspace(&mut self) {
        self.model_picker.model_filter_backspace();
    }

    pub fn selected_model(&self) -> Option<&str> {
        self.model_picker.selected_model()
    }
}
