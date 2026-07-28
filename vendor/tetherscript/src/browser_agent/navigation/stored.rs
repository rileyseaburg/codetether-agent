//! Stored page history entry state.

use crate::browser_agent::navigation::result::NavigationKind;
use crate::browser_session::BrowserSession;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct StoredHistoryEntry {
    pub(super) session: BrowserSession,
    pub(super) navigation_id: u64,
    pub(super) kind: NavigationKind,
}

impl StoredHistoryEntry {
    pub(super) fn new(session: BrowserSession, navigation_id: u64, kind: NavigationKind) -> Self {
        Self {
            session,
            navigation_id,
            kind,
        }
    }
}
