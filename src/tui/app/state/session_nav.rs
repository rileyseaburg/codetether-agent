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
        if self.session_filter.is_empty() {
            self.sessions.iter().enumerate().collect()
        } else {
            let filter = self.session_filter.to_lowercase();
            self.sessions
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    s.title
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&filter)
                        || s.id.to_lowercase().contains(&filter)
                })
                .collect()
        }
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
