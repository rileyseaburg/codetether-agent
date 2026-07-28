//! Service-worker store container.

use super::{CacheStore, ServiceWorkerFetchEvent, ServiceWorkerRegistration};

/// Context-scoped service-worker registrations and CacheStorage.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ServiceWorkerStore {
    pub(crate) registrations: Vec<ServiceWorkerRegistration>,
    pub(crate) caches: CacheStore,
    pub(crate) fetch_log: Vec<ServiceWorkerFetchEvent>,
    pub(crate) next_sequence: u64,
}

impl ServiceWorkerStore {
    /// Create an empty service-worker store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return true when no registrations or cache records exist.
    pub fn is_empty(&self) -> bool {
        self.registrations.is_empty() && self.caches.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.registrations.clear();
        self.caches.clear();
        self.fetch_log.clear();
        self.next_sequence = 0;
    }
}
