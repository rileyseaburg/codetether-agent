//! Application-local session-picker state adapters.
pub use codetether_session_metadata::SessionSummary;
/// Declare the existing session-picker methods on a local AppState.
/// # Usage
/// Pass the state type and any native type/variant bindings required by the matcher.
#[macro_export]
macro_rules! impl_app_session_picker {
    ($state:ty) => {
        impl $state {
            pub fn sessions_select_prev(&mut self) {
                self.session_picker.sessions_select_prev()
            }
            pub fn sessions_select_next(&mut self) {
                self.session_picker.sessions_select_next()
            }
            pub fn filtered_sessions(
                &self,
            ) -> Vec<(usize, &$crate::state_binding::SessionSummary)> {
                self.session_picker.filtered_sessions()
            }
            pub fn clear_session_filter(&mut self) {
                self.session_picker.clear_session_filter()
            }
            pub fn session_filter_backspace(&mut self) {
                self.session_picker.session_filter_backspace()
            }
            pub fn session_filter_push(&mut self, c: char) {
                self.session_picker.session_filter_push(c)
            }
        }
    };
}
