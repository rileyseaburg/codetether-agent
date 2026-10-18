//! Query editing and access to ranked session matches.
use crate::SessionPickerState;
use codetether_session_metadata::SessionSummary;
impl SessionPickerState {
    /// Borrow ranked matches from the cached summaries.
    pub fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
        crate::filtered_sessions(&self.sessions, &self.session_filter)
    }
    /// Clear the query and reset the cursor.
    pub fn clear_session_filter(&mut self) {
        self.session_filter.clear();
        self.selected_session = 0;
    }
    /// Remove one query character and reset the cursor.
    pub fn session_filter_backspace(&mut self) {
        self.session_filter.pop();
        self.selected_session = 0;
    }
    /// Append a query character and reset the cursor.
    pub fn session_filter_push(&mut self, c: char) {
        self.session_filter.push(c);
        self.selected_session = 0;
    }
}
