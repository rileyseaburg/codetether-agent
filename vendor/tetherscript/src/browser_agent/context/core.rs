//! Core browser context construction and page access.

use super::context_state::shared_context_state;
use super::BrowserContext;
use crate::browser_agent::page::BrowserPage;

impl Default for BrowserContext {
    fn default() -> Self {
        Self {
            pages: Vec::new(),
            state: shared_context_state(),
            security_policy: super::super::exports::SecurityPolicy::default(),
            permissions: super::super::permissions::PermissionStore::default(),
            geolocation: super::super::permissions::GeolocationEmulation::default(),
        }
    }
}

impl BrowserContext {
    /// Create an empty browser context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a page and return its index.
    pub fn new_page(&mut self, mut page: BrowserPage) -> usize {
        let adopt_page_state = self.pages.is_empty() && self.state.borrow().is_empty();
        page.attach_context_state(self.state.clone(), adopt_page_state);
        page.set_security_policy(self.security_policy.clone());
        page.replace_permissions(self.permissions.clone());
        page.geolocation = self.geolocation.clone();
        self.pages.push(page);
        self.pages.len() - 1
    }

    /// Get a page by index.
    pub fn page(&self, index: usize) -> Option<&BrowserPage> {
        self.pages.get(index)
    }

    /// Get a mutable page by index.
    pub fn page_mut(&mut self, index: usize) -> Option<&mut BrowserPage> {
        let page = self.pages.get_mut(index)?;
        page.sync_context_state_into_session();
        Some(page)
    }

    /// Return the number of pages in the context.
    pub fn len(&self) -> usize {
        self.pages.len()
    }

    /// Return true when the context has no pages.
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
}
