//! Bounded command-history insertion.

const MAX_ITEMS: usize = 100;
const MAX_ENTRY_BYTES: usize = 64 * 1024;

impl super::super::AppState {
    pub fn push_history(&mut self, entry: String) {
        if entry.trim().is_empty() {
            return;
        }
        let entry = if entry.len() > MAX_ENTRY_BYTES {
            crate::util::truncate_bytes_safe(&entry, MAX_ENTRY_BYTES).to_string()
        } else {
            entry
        };
        self.command_history.push(entry);
        let overflow = self.command_history.len().saturating_sub(MAX_ITEMS);
        self.command_history.drain(..overflow);
        self.history_index = None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn history_caps_entries_and_payload_bytes() {
        let mut state = crate::tui::app::state::AppState::default();
        for index in 0..150 {
            state.push_history(index.to_string());
        }
        state.push_history("x".repeat(super::MAX_ENTRY_BYTES + 1));
        assert_eq!(state.command_history.len(), super::MAX_ITEMS);
        assert!(state.command_history.last().unwrap().len() <= super::MAX_ENTRY_BYTES);
    }
}
