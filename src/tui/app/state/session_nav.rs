//! Compatibility methods forwarding to owned session-picker state.
use crate::session::SessionSummary;
impl super::AppState {
    pub fn sessions_select_prev(&mut self) {
        self.session_picker.sessions_select_prev()
    }
    pub fn sessions_select_next(&mut self) {
        self.session_picker.sessions_select_next()
    }
    pub fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
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
