//! Page-level DOM storage helpers.

use super::origin::storage_origin;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Set a current-origin localStorage item and update attached contexts.
    pub fn set_local_storage_item(&mut self, key: &str, value: &str) {
        self.sync_context_state_into_session();
        self.session.set_local_storage(key, value);
        self.sync_context_state_from_session();
    }

    /// Return a current-origin localStorage item.
    pub fn local_storage_item(&mut self, key: &str) -> Option<String> {
        self.sync_context_state_into_session();
        self.session.local_storage_item(key).map(str::to_string)
    }

    /// Clear current-origin localStorage and update attached contexts.
    pub fn clear_local_storage(&mut self) {
        let origin = storage_origin(&self.session.url);
        self.sync_context_state_into_session();
        self.session.local_storage.remove(&origin);
        self.sync_context_state_from_session();
        self.runtime = None;
    }

    /// Set a current-origin sessionStorage item for this page only.
    pub fn set_session_storage_item(&mut self, key: &str, value: &str) {
        self.session.set_session_storage(key, value);
    }

    /// Return a current-origin sessionStorage item for this page.
    pub fn session_storage_item(&self, key: &str) -> Option<String> {
        self.session.session_storage_item(key).map(str::to_string)
    }

    /// Clear current-origin sessionStorage for this page only.
    pub fn clear_session_storage(&mut self) {
        let origin = storage_origin(&self.session.url);
        self.session.session_storage.remove(&origin);
        self.runtime = None;
    }
}
