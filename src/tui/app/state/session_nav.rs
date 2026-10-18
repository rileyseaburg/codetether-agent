//! Session list navigation and filtering methods.

use crate::session::SessionSummary;

impl super::AppState {
    pub fn sessions_select_prev(&mut self) {
        if self.selected_session > 0 {
            self.selected_session -= 1;
        }
    }

    pub fn sessions_select_next(&mut self) {
        if self.selected_session + 1 < self.filtered_sessions().len() {
            self.selected_session += 1;
        }
    }

    pub fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
        codetether_session_picker::filtered_sessions(&self.sessions, &self.session_filter)
    }

    pub fn clear_session_filter(&mut self) {
        self.session_filter.clear();
        self.selected_session = 0;
    }

    pub fn session_filter_backspace(&mut self) {
        self.session_filter.pop();
        self.selected_session = 0;
    }

    pub fn session_filter_push(&mut self, c: char) {
        self.session_filter.push(c);
        self.selected_session = 0;
    }
}
