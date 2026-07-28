//! Page-origin service-worker and CacheStorage APIs.

use super::{origin, ServiceWorkerFetchEvent, ServiceWorkerRegistration};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Register a worker for the page origin.
    pub fn service_worker_register(
        &mut self,
        scope: &str,
        script_url: &str,
    ) -> Result<ServiceWorkerRegistration, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        Ok(self
            .service_worker_state()?
            .borrow_mut()
            .service_workers
            .register(&origin, scope, script_url))
    }

    /// Activate a worker for the page origin.
    pub fn service_worker_activate(&mut self, scope: &str) -> Result<bool, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        Ok(self
            .service_worker_state()?
            .borrow_mut()
            .service_workers
            .activate(&origin, scope))
    }

    /// Return fetch interception metadata for the owning context.
    pub fn service_worker_fetch_log(&self) -> Result<Vec<ServiceWorkerFetchEvent>, String> {
        Ok(self
            .service_worker_state()?
            .borrow()
            .service_workers
            .fetch_log())
    }

    /// Return service-worker registrations for the page origin.
    pub fn service_worker_registrations(&self) -> Result<Vec<ServiceWorkerRegistration>, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        Ok(self
            .service_worker_state()?
            .borrow()
            .service_workers
            .registrations(&origin))
    }
}
