//! Resolve the selected filtered row to its durable session identity.
use crate::SessionPickerState;
impl SessionPickerState {
    /// Clone the selected session ID from the ranked listing, if present.
    /// # Examples
    /// ```
    /// assert!(codetether_session_picker::SessionPickerState::default().selected_session_id().is_none());
    /// ```
    pub fn selected_session_id(&self) -> Option<String> {
        self.filtered_sessions()
            .get(self.selected_session)
            .map(|(orig_idx, _)| self.sessions[*orig_idx].id.clone())
    }
}
