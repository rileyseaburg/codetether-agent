//! Synchronous model picker navigation and filter methods.

impl super::AppState {
    pub fn filtered_models(&self) -> Vec<&str> {
        if self.model_filter.is_empty() {
            self.available_models.iter().map(String::as_str).collect()
        } else {
            let filter = self.model_filter.to_lowercase();
            self.available_models
                .iter()
                .map(String::as_str)
                .filter(|model| model.to_lowercase().contains(&filter))
                .collect()
        }
    }

    pub fn set_available_models(&mut self, models: Vec<String>) {
        self.available_models = models;
        if self.selected_model_index >= self.filtered_models().len() {
            self.selected_model_index = self.filtered_models().len().saturating_sub(1);
        }
    }

    pub fn open_model_picker(&mut self) {
        self.model_picker_active = true;
        self.model_filter.clear();
        self.selected_model_index = 0;
        self.status = "Model picker".to_string();
    }

    pub fn close_model_picker(&mut self) {
        self.model_picker_active = false;
        self.model_filter.clear();
        self.selected_model_index = 0;
    }

    pub fn model_select_prev(&mut self) {
        self.selected_model_index = self.selected_model_index.saturating_sub(1);
    }

    pub fn model_select_next(&mut self) {
        if self.selected_model_index + 1 < self.filtered_models().len() {
            self.selected_model_index += 1;
        }
    }

    pub fn model_filter_push(&mut self, c: char) {
        self.model_filter.push(c);
        self.selected_model_index = 0;
    }

    pub fn model_filter_backspace(&mut self) {
        self.model_filter.pop();
        self.selected_model_index = 0;
    }

    pub fn selected_model(&self) -> Option<&str> {
        self.filtered_models()
            .get(self.selected_model_index)
            .copied()
    }
}
