//! Filtering behavior for bus log state.

use super::{entry::BusLogEntry, state::BusLogState};

impl BusLogState {
    /// Returns entries matching the current filter.
    pub fn filtered_entries(&self) -> Vec<&BusLogEntry> {
        if self.filter.trim().is_empty() {
            return self.entries.iter().collect();
        }
        let needle = self.filter.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| {
                entry.topic.to_lowercase().contains(&needle)
                    || entry.sender_id.to_lowercase().contains(&needle)
                    || entry.kind.to_lowercase().contains(&needle)
                    || entry.summary.to_lowercase().contains(&needle)
            })
            .collect()
    }

    /// Returns the number of entries visible after filtering.
    pub fn visible_count(&self) -> usize {
        self.filtered_entries().len()
    }

    /// Starts editing the bus log filter.
    pub fn enter_filter_mode(&mut self) {
        self.filter_input_mode = true;
    }

    /// Stops editing the bus log filter.
    pub fn exit_filter_mode(&mut self) {
        self.filter_input_mode = false;
    }

    /// Clears the filter and selects the latest visible entry.
    pub fn clear_filter(&mut self) {
        self.filter.clear();
        self.selected_index = self.visible_count().saturating_sub(1);
    }

    /// Appends one character to the filter.
    pub fn push_filter_char(&mut self, c: char) {
        self.filter.push(c);
        self.selected_index = 0;
    }

    /// Removes one character from the filter.
    pub fn pop_filter_char(&mut self) {
        self.filter.pop();
        self.selected_index = 0;
    }
}
