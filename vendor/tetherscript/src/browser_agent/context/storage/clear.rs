//! Context storage clearing helpers.

use super::super::BrowserContext;
use super::sync::sync_pages;

impl BrowserContext {
    /// Clear shared storage plus open page sessionStorage buckets.
    pub fn clear_storage_state(&mut self) {
        {
            let mut state = self.state.borrow_mut();
            state.cookies.clear();
            state.local_storage.clear();
            state.indexed_db.clear();
            state.service_workers.clear();
        }
        for page in &mut self.pages {
            page.session.session_storage.clear();
        }
        sync_pages(self);
    }
}
