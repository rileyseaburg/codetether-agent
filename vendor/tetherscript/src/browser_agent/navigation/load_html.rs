//! `BrowserPage::load_html` implementation.

use crate::browser_agent::navigation::result::{NavigationKind, NavigationResult};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub fn load_html(&mut self, html: impl Into<String>) -> NavigationResult {
        let from = self.session.url.clone();
        let log = super::load_initial::should_log(self, &from);
        if log {
            super::super::lifecycle::leave(
                self,
                "load_html",
                NavigationKind::DocumentReplacement,
                &from,
                &from,
            );
        }
        self.session.load_html(html);
        self.runtime = None;
        let result =
            super::super::load_api::replace(self, "load_html", NavigationKind::DocumentReplacement);
        if log {
            super::super::lifecycle::finish_commit(
                self,
                "load_html",
                result.kind,
                &from,
                &result.navigation.url,
            );
        }
        result
    }
}
