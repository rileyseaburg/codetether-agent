//! Service-worker registry read operations.

use super::{ServiceWorkerRegistration, ServiceWorkerStore};

impl ServiceWorkerStore {
    /// Return registrations for one origin in deterministic order.
    pub fn registrations(&self, origin: &str) -> Vec<ServiceWorkerRegistration> {
        let mut registrations: Vec<_> = self
            .registrations
            .iter()
            .filter(|registration| registration.origin == origin)
            .cloned()
            .collect();
        registrations.sort();
        registrations
    }

    /// Return every registration in deterministic order.
    pub fn all_registrations(&self) -> Vec<ServiceWorkerRegistration> {
        let mut registrations = self.registrations.clone();
        registrations.sort();
        registrations
    }

    pub(crate) fn active_for(&self, origin: &str, url: &str) -> Option<ServiceWorkerRegistration> {
        let mut registrations: Vec<_> = self
            .registrations
            .iter()
            .filter(|item| item.origin == origin && item.state.is_active())
            .filter(|item| url.starts_with(&item.scope))
            .cloned()
            .collect();
        registrations.sort_by(|left, right| right.scope.len().cmp(&left.scope.len()));
        registrations.into_iter().next()
    }
}
