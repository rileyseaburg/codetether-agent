//! Page-origin CacheStorage key and delete APIs.

use super::origin;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Delete a cache response for the page origin.
    pub fn cache_delete(&mut self, cache: &str, request_url: &str) -> Result<bool, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        let url = origin::service_worker_url(&origin, request_url);
        Ok(self
            .service_worker_state()?
            .borrow_mut()
            .service_workers
            .cache_delete(&origin, cache, &url))
    }

    /// List request URL keys for one page-origin cache.
    pub fn cache_keys(&self, cache: &str) -> Result<Vec<String>, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        Ok(self
            .service_worker_state()?
            .borrow()
            .service_workers
            .cache_keys(&origin, cache))
    }
}
