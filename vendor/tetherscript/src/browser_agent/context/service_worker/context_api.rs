//! Browser context service-worker and CacheStorage APIs.

use super::{origin, ServiceWorkerFetchEvent, ServiceWorkerRegistration};
use crate::browser_agent::context::BrowserContext;

impl BrowserContext {
    /// Register a service worker for an origin and scope.
    pub fn service_worker_register(
        &mut self,
        origin_or_url: &str,
        scope: &str,
        script_url: &str,
    ) -> ServiceWorkerRegistration {
        let origin = origin::service_worker_origin(origin_or_url);
        self.state
            .borrow_mut()
            .service_workers
            .register(&origin, scope, script_url)
    }

    /// Activate a matching service-worker registration.
    pub fn service_worker_activate(&mut self, origin_or_url: &str, scope: &str) -> bool {
        let origin = origin::service_worker_origin(origin_or_url);
        self.state
            .borrow_mut()
            .service_workers
            .activate(&origin, scope)
    }

    /// Return registrations for one origin.
    pub fn service_worker_registrations(
        &self,
        origin_or_url: &str,
    ) -> Vec<ServiceWorkerRegistration> {
        let origin = origin::service_worker_origin(origin_or_url);
        self.state.borrow().service_workers.registrations(&origin)
    }

    /// Return deterministic service-worker fetch metadata.
    pub fn service_worker_fetch_log(&self) -> Vec<ServiceWorkerFetchEvent> {
        self.state.borrow().service_workers.fetch_log()
    }
}
