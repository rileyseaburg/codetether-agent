//! `BrowserPage::reload` implementation.

use crate::browser_agent::navigation::result::{NavigationKind, NavigationResult};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub fn reload(&mut self) -> NavigationResult {
        let from = self.session.url.clone();
        super::super::lifecycle::leave(self, "reload", NavigationKind::Reload, &from, &from);
        self.session.reload();
        self.runtime = None;
        let result = super::super::load_api::replace(self, "reload", NavigationKind::Reload);
        super::super::lifecycle::finish_commit(
            self,
            "reload",
            result.kind,
            &from,
            &result.navigation.url,
        );
        result
    }
}
