//! Application-local model-picker state adapters.

/// Declare the existing model-picker methods on a local AppState.
/// # Usage
/// Pass the state type and any native type/variant bindings required by the matcher.
#[macro_export]
macro_rules! __app_model_navigation {
    ($state:ty) => {
        impl $state {
            pub fn filtered_models(&self) -> Vec<&str> {
                self.model_picker.filtered_models()
            }

            pub fn set_available_models(&mut self, models: ::std::vec::Vec<::std::string::String>) {
                self.model_picker.set_available_models(models);
            }

            pub fn open_model_picker(&mut self) {
                self.status = self.model_picker.open_with_status();
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
    };
}
