//! Page history entry listing.

use super::entry::PageHistoryEntry;
use super::store::PageHistory;

impl PageHistory {
    pub(crate) fn entries(&self) -> Vec<PageHistoryEntry> {
        self.entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                PageHistoryEntry::new(
                    index,
                    entry.session.url.clone(),
                    entry.navigation_id,
                    entry.kind,
                    index == self.index,
                )
            })
            .collect()
    }
}
