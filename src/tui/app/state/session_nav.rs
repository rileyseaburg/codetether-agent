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
            return self.sessions.iter().enumerate().collect();
        }
        let needle = self.session_filter.as_str();
        let mut scored: Vec<(u32, usize, &SessionSummary)> = self
            .sessions
            .iter()
            .enumerate()
            .filter_map(|(i, s)| best_score(needle, s).map(|sc| (sc, i, s)))
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        scored.into_iter().map(|(_, i, s)| (i, s)).collect()
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

/// Best fuzzy score for `needle` across a session's id and title.
fn best_score(needle: &str, s: &SessionSummary) -> Option<u32> {
    let id = super::session_fuzzy::fuzzy_score(needle, &s.id);
    let title = s
        .title
        .as_deref()
        .and_then(|t| super::session_fuzzy::fuzzy_score(needle, t));
    id.into_iter().chain(title).max()
}
