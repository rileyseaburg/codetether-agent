//! Native localStorage access for browser contexts.

use super::super::BrowserContext;
use super::origin::storage_origin;
use super::sync::sync_pages;

impl BrowserContext {
    /// Set a shared localStorage item for an origin or URL.
    pub fn set_local_storage_item(&mut self, origin_or_url: &str, key: &str, value: &str) {
        let origin = storage_origin(origin_or_url);
        self.state
            .borrow_mut()
            .local_storage
            .entry(origin)
            .or_default()
            .insert(key.into(), value.into());
        sync_pages(self);
    }

    /// Return a shared localStorage item for an origin or URL.
    pub fn local_storage_item(&self, origin_or_url: &str, key: &str) -> Option<String> {
        let origin = storage_origin(origin_or_url);
        self.state
            .borrow()
            .local_storage
            .get(&origin)?
            .get(key)
            .cloned()
    }

    /// Clear one origin-scoped localStorage bucket.
    pub fn clear_local_storage(&mut self, origin_or_url: &str) {
        let origin = storage_origin(origin_or_url);
        self.state.borrow_mut().local_storage.remove(&origin);
        sync_pages(self);
    }
}
