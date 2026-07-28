//! Context-level storage-state snapshot and restore APIs.

use super::super::persistence::BrowserStorageState;
use super::super::BrowserContext;
use super::sync::sync_pages;
use crate::browser_agent::context::persistence::{restore_storage, snapshot_storage};
use crate::browser_cookie;

impl BrowserContext {
    /// Capture cookies, localStorage, and IndexedDB without open pages.
    pub fn storage_state(&self) -> BrowserStorageState {
        let state = self.state.borrow();
        BrowserStorageState {
            cookies: browser_cookie::persistent_cookies(&state.cookies),
            local_storage: snapshot_storage(&state.local_storage),
            indexed_db: state.indexed_db.list_all(),
            service_workers: state.service_workers.all_registrations(),
            caches: state.service_workers.cache_records(),
        }
    }

    /// Replace shared storage state and sync existing pages.
    pub fn restore_storage_state(&mut self, snapshot: BrowserStorageState) {
        {
            let mut state = self.state.borrow_mut();
            state.cookies = snapshot.cookies;
            state.local_storage = restore_storage(&snapshot.local_storage);
            state.indexed_db.replace_all(snapshot.indexed_db);
            state
                .service_workers
                .replace_all(snapshot.service_workers, snapshot.caches);
        }
        sync_pages(self);
    }
}
