//! Service-worker registration operations.

use super::origin::service_worker_url;
use super::{ServiceWorkerRegistration, ServiceWorkerState, ServiceWorkerStore};

impl ServiceWorkerStore {
    /// Register or replace one worker by origin and scope.
    pub fn register(
        &mut self,
        origin: &str,
        scope: &str,
        script_url: &str,
    ) -> ServiceWorkerRegistration {
        let scope = service_worker_url(origin, scope);
        let script_url = service_worker_url(origin, script_url);
        let registration = ServiceWorkerRegistration::new(origin, scope, script_url);
        if let Some(existing) = self.same_scope_mut(&registration) {
            *existing = registration.clone();
        } else {
            self.registrations.push(registration.clone());
        }
        registration
    }

    /// Activate one registered worker.
    pub fn activate(&mut self, origin: &str, scope: &str) -> bool {
        let scope = service_worker_url(origin, scope);
        let Some(registration) = self
            .registrations
            .iter_mut()
            .find(|item| item.origin == origin && item.scope == scope)
        else {
            return false;
        };
        registration.state = ServiceWorkerState::Active;
        true
    }

    fn same_scope_mut(
        &mut self,
        registration: &ServiceWorkerRegistration,
    ) -> Option<&mut ServiceWorkerRegistration> {
        self.registrations
            .iter_mut()
            .find(|item| item.origin == registration.origin && item.scope == registration.scope)
    }
}
