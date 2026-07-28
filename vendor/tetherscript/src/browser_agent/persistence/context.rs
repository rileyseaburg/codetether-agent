//! Context snapshot and restore APIs.

use super::storage::{restore_storage, snapshot_storage};
use super::types::BrowserContextSnapshot;
use crate::browser_agent::context::context_state::shared_context_state;
use crate::browser_agent::{BrowserContext, BrowserPage};
use crate::browser_cookie;
use crate::browser_session::BrowserSession;

impl BrowserContext {
    /// Capture shared context state and all pages without cloning JS heaps.
    ///
    /// # Returns
    ///
    /// A native Rust snapshot of shared cookies, shared localStorage, and all
    /// current pages in context order.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserContext;
    ///
    /// let context = BrowserContext::new();
    /// assert!(context.snapshot_state().pages.is_empty());
    /// ```
    pub fn snapshot_state(&self) -> BrowserContextSnapshot {
        let state = self.state.borrow();
        BrowserContextSnapshot {
            cookies: browser_cookie::persistent_cookies(&state.cookies),
            local_storage: snapshot_storage(&state.local_storage),
            indexed_db: state.indexed_db.list_all(),
            service_workers: state.service_workers.all_registrations(),
            caches: state.service_workers.cache_records(),
            permissions: self.permissions.grants(),
            geolocation: self.geolocation.clone(),
            pages: self.pages.iter().map(BrowserPage::snapshot_state).collect(),
        }
    }

    /// Restore shared context state and pages with fresh page runtimes.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - Native context snapshot previously returned by
    ///   [`BrowserContext::snapshot_state`].
    ///
    /// # Errors
    ///
    /// Returns `Err` when any page snapshot contains invalid viewport or device
    /// scale metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserContext;
    ///
    /// let snapshot = BrowserContext::new().snapshot_state();
    /// let mut restored = BrowserContext::new();
    /// restored.restore_state(snapshot).unwrap();
    /// assert!(restored.is_empty());
    /// ```
    pub fn restore_state(&mut self, snapshot: BrowserContextSnapshot) -> Result<(), String> {
        let shared = shared_context_state();
        {
            let mut state = shared.borrow_mut();
            state.cookies = snapshot.cookies;
            state.local_storage = restore_storage(&snapshot.local_storage);
            state.indexed_db.replace_all(snapshot.indexed_db);
            state
                .service_workers
                .replace_all(snapshot.service_workers, snapshot.caches);
        }
        self.pages.clear();
        self.state = shared;
        self.permissions.replace_all(snapshot.permissions);
        self.geolocation = snapshot.geolocation;
        for page_snapshot in snapshot.pages {
            let mut page = BrowserPage::new(BrowserSession::new());
            page.restore_state(page_snapshot)?;
            page.attach_context_state(self.state.clone(), false);
            self.pages.push(page);
        }
        Ok(())
    }
}
