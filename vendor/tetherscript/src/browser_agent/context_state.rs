//! Context-level cookie and storage sharing for agent pages.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::indexed_db::IndexedDbStore;
use super::service_worker::ServiceWorkerStore;
use crate::browser_session::{BrowserSession, Cookie};

/// Shared mutable browser context state.
pub type SharedContextState = Rc<RefCell<BrowserContextState>>;

/// Cookie and localStorage data shared by pages in one context.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BrowserContextState {
    pub cookies: Vec<Cookie>,
    pub local_storage: HashMap<String, HashMap<String, String>>,
    pub indexed_db: IndexedDbStore,
    pub service_workers: ServiceWorkerStore,
}

impl BrowserContextState {
    /// Create empty context state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return true when no shared browser state has been recorded.
    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
            && self.local_storage.is_empty()
            && self.indexed_db.is_empty()
            && self.service_workers.is_empty()
    }

    /// Copy shared cookies and localStorage into a page session.
    pub fn apply_to_session(&self, session: &mut BrowserSession) {
        session.cookies = self.cookies.clone();
        session.local_storage = self.local_storage.clone();
    }

    /// Replace shared cookies and localStorage from a page session.
    pub fn absorb_session(&mut self, session: &BrowserSession) {
        self.cookies = session.cookies.clone();
        self.local_storage = session.local_storage.clone();
    }
}

/// Create reference-counted context state for sharing between pages.
pub fn shared_context_state() -> SharedContextState {
    Rc::new(RefCell::new(BrowserContextState::new()))
}
