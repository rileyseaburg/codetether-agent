//! Listing replacement and active-session upserts.
use crate::SessionPickerState;
use codetether_session_metadata::SessionSummary;
impl SessionPickerState {
    /// Replace cached summaries and clamp selection to the filtered list.
    pub fn replace_sessions(&mut self, sessions: Vec<SessionSummary>) {
        self.sessions = sessions;
        if self.selected_session >= self.filtered_sessions().len() {
            self.selected_session = self.filtered_sessions().len().saturating_sub(1);
        }
    }
    /// Clear cached summaries and reset selection, preserving the query.
    pub fn clear_sessions(&mut self) {
        self.sessions.clear();
        self.selected_session = 0;
    }
    /// Move fresh metadata to the first row, removing an older matching entry.
    pub fn upsert_session(&mut self, summary: SessionSummary) {
        let sessions = &mut self.sessions;
        if let Some(index) = sessions.iter().position(|item| item.id == summary.id) {
            sessions.remove(index);
        }
        sessions.insert(0, summary);
        self.selected_session = 0;
    }
}
