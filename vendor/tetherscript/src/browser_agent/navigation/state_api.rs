//! Page navigation state accessors.

use crate::browser_agent::navigation_state::{PageLoadState, PageNavigation};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub fn navigation(&self) -> &PageNavigation {
        &self.navigation
    }

    pub fn load_state(&self) -> PageLoadState {
        self.navigation.state
    }

    pub fn wait_for_load_state(&self, state: PageLoadState) -> Result<PageNavigation, String> {
        if self.load_state().reached(state) {
            Ok(self.navigation.clone())
        } else {
            Err(format!(
                "page at {} is in {:?}, not {:?}",
                self.navigation.url, self.navigation.state, state
            ))
        }
    }

    pub(crate) fn mark_load_state(&mut self, state: PageLoadState) {
        self.navigation.advance(state);
    }
}
