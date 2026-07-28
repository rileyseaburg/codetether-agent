//! `BrowserPage::goto_html` implementation.

use crate::browser_agent::navigation::result::{NavigationKind, NavigationResult};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub fn goto_html(
        &mut self,
        url: impl Into<String>,
        html: impl Into<String>,
    ) -> NavigationResult {
        let from = self.session.url.clone();
        let url = url.into();
        let log = super::load_initial::should_log(self, &from);
        if log {
            super::super::lifecycle::leave(
                self,
                "goto_html",
                NavigationKind::DocumentReplacement,
                &from,
                &url,
            );
        }
        self.session.goto_html(url.clone(), html);
        self.runtime = None;
        let result =
            super::super::load_api::push(self, "goto_html", NavigationKind::DocumentReplacement);
        if log {
            super::super::lifecycle::finish_commit(
                self,
                "goto_html",
                result.kind,
                &from,
                &result.navigation.url,
            );
        }
        result
    }
}
