//! Service-worker snapshot replacement helpers.

use super::{CacheRecord, ServiceWorkerRegistration, ServiceWorkerStore};

impl ServiceWorkerStore {
    pub(crate) fn replace_all(
        &mut self,
        registrations: Vec<ServiceWorkerRegistration>,
        caches: Vec<CacheRecord>,
    ) {
        self.clear();
        self.registrations = registrations;
        self.caches.records = caches;
    }
}
