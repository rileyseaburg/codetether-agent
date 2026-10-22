//! Bounded, coalesced runtime-event history for the live chat footer.
use std::{collections::VecDeque, time::SystemTime};

/// Latest event groups, kept separate from persisted chat messages.
#[derive(Default)]
pub(crate) struct ActivityEvents {
    pub(crate) entries: VecDeque<ActivityEntry>,
}

/// A run of identical events and the time its latest update arrived.
pub(crate) struct ActivityEntry {
    pub(crate) label: String,
    pub(crate) count: u64,
    pub(crate) at: SystemTime,
}

impl ActivityEvents {
    /// Record an event, coalescing repeats and retaining four groups.
    pub(crate) fn record(&mut self, label: String) {
        let at = SystemTime::now();
        if let Some(last) = self.entries.back_mut()
            && last.label == label
        {
            last.count = last.count.saturating_add(1);
            last.at = at;
            return;
        }
        if self.entries.len() == 4 {
            self.entries.pop_front();
        }
        self.entries.push_back(ActivityEntry {
            label,
            count: 1,
            at,
        });
    }
}
