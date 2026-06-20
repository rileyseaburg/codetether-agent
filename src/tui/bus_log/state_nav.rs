//! Selection and detail navigation for bus log state.

use crate::bus::BusEnvelope;

use super::{entry::BusLogEntry, state::BusLogState};

impl BusLogState {
    /// Creates an empty bus log state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes one render-ready entry into the retained log.
    pub fn push(&mut self, entry: BusLogEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            let overflow = self.entries.len() - self.max_entries;
            self.entries.drain(0..overflow);
            self.selected_index = self.selected_index.saturating_sub(overflow);
        }
        if self.auto_scroll {
            self.selected_index = self.visible_count().saturating_sub(1);
        }
    }

    /// Converts and ingests one bus envelope.
    pub fn ingest(&mut self, env: &BusEnvelope) {
        self.push(BusLogEntry::from_envelope(env));
    }

    /// Selects the previous visible entry.
    pub fn select_prev(&mut self) {
        self.auto_scroll = false;
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    /// Selects the next visible entry.
    pub fn select_next(&mut self) {
        self.auto_scroll = false;
        let max_index = self.visible_count().saturating_sub(1);
        if self.selected_index < max_index {
            self.selected_index += 1;
        }
    }
}
