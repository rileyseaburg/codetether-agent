//! Page history traversal APIs.

use crate::browser_agent::navigation::result::NavigationResult;
use crate::browser_agent::navigation_state::PageLoadState;
use crate::browser_agent::page::BrowserPage;

use super::entry::PageHistoryEntry;

impl BrowserPage {
    pub fn history_entries(&self) -> Vec<PageHistoryEntry> {
        self.history.entries()
    }

    pub fn history_index(&self) -> usize {
        self.history.index()
    }

    pub fn go_back(&mut self) -> NavigationResult {
        self.go_by(-1, "go_back")
    }

    pub fn go_forward(&mut self) -> NavigationResult {
        self.go_by(1, "go_forward")
    }

    pub fn go(&mut self, delta: isize) -> NavigationResult {
        if delta == 0 {
            return self.reload();
        }
        self.go_by(delta, "history_go")
    }

    fn go_by(&mut self, delta: isize, action: &str) -> NavigationResult {
        let from = self.session.url.clone();
        let Some(session) = self.history.go(delta) else {
            super::lifecycle::no_entry(self, action, &from);
            return NavigationResult::no_entry(self.navigation.clone());
        };
        let to = session.url.clone();
        let kind = super::url_kind::transition(&from, &to);
        if !matches!(kind, super::result::NavigationKind::SameDocument) {
            super::lifecycle::leave(self, action, kind, &from, &to);
        }
        self.session = session;
        if !matches!(kind, super::result::NavigationKind::SameDocument) {
            self.runtime = None;
        }
        self.navigation =
            self.navigation
                .next(self.session.url.clone(), action, PageLoadState::Load);
        self.history
            .replace_current(self.session.clone(), self.navigation.id, kind);
        super::lifecycle::finish_history(self, action, kind, &from, &to);
        NavigationResult::committed(self.navigation.clone(), kind)
    }
}
