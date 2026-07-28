//! Page-local history cursor storage.

use crate::browser_session::BrowserSession;

use super::result::NavigationKind;
use super::stored::StoredHistoryEntry;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageHistory {
    pub(super) entries: Vec<StoredHistoryEntry>,
    pub(super) index: usize,
}

impl PageHistory {
    pub(crate) fn new(session: BrowserSession, navigation_id: u64) -> Self {
        Self {
            entries: vec![StoredHistoryEntry::new(
                session,
                navigation_id,
                NavigationKind::Initial,
            )],
            index: 0,
        }
    }

    pub(crate) fn replace_current(
        &mut self,
        session: BrowserSession,
        navigation_id: u64,
        kind: NavigationKind,
    ) {
        self.entries[self.index] = StoredHistoryEntry::new(session, navigation_id, kind);
    }

    pub(crate) fn push(&mut self, session: BrowserSession, id: u64, kind: NavigationKind) {
        self.entries.truncate(self.index + 1);
        self.entries
            .push(StoredHistoryEntry::new(session, id, kind));
        self.index = self.entries.len() - 1;
    }

    pub(crate) fn go(&mut self, delta: isize) -> Option<BrowserSession> {
        let target = self.index as isize + delta;
        if target < 0 || target as usize >= self.entries.len() {
            return None;
        }
        self.index = target as usize;
        Some(self.entries[self.index].session.clone())
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}
