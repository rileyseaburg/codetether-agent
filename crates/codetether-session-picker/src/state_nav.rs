//! Cursor movement within filtered session rows.
use crate::SessionPickerState;
impl SessionPickerState {
    /// Move the cursor to the previous row, stopping at zero.
    pub fn sessions_select_prev(&mut self) {
        if self.selected_session > 0 {
            self.selected_session -= 1;
        }
    }
    /// Move the cursor forward while a filtered row remains.
    pub fn sessions_select_next(&mut self) {
        if self.selected_session + 1 < self.filtered_sessions().len() {
            self.selected_session += 1;
        }
    }
}
