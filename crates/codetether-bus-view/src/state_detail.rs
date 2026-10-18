//! Detail-mode behavior for bus log state.

use super::{entry::BusLogEntry, state::BusLogState};

impl BusLogState {
    /// Opens the selected entry's detail pane.
    pub fn enter_detail(&mut self) {
        self.detail_mode = true;
        self.detail_scroll = 0;
    }

    /// Closes the detail pane.
    pub fn exit_detail(&mut self) {
        self.detail_mode = false;
        self.detail_scroll = 0;
    }

    /// Scrolls detail text upward.
    pub fn detail_scroll_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    /// Scrolls detail text downward.
    pub fn detail_scroll_down(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }

    /// Returns the currently selected visible entry.
    pub fn selected_entry(&self) -> Option<&BusLogEntry> {
        self.filtered_entries().get(self.selected_index).copied()
    }

    /// Counts retained A2A free-form messages.
    pub fn a2a_message_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind == "A2A•MSG")
            .count()
    }
}
