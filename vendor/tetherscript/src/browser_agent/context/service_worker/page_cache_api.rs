//! Page-origin CacheStorage APIs.

use super::{origin, CacheRecord, CacheResponse};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Insert or replace a cache response for the page origin.
    pub fn cache_put(
        &mut self,
        cache: &str,
        request_url: &str,
        response: CacheResponse,
    ) -> Result<(), String> {
        let origin = origin::service_worker_origin(&self.session.url);
        let url = origin::service_worker_url(&origin, request_url);
        self.service_worker_state()?
            .borrow_mut()
            .service_workers
            .cache_put(&origin, cache, &url, response);
        Ok(())
    }

    /// Match a cache response for the page origin.
    pub fn cache_match(
        &self,
        cache: &str,
        request_url: &str,
    ) -> Result<Option<CacheResponse>, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        let url = origin::service_worker_url(&origin, request_url);
        Ok(self
            .service_worker_state()?
            .borrow()
            .service_workers
            .cache_match(&origin, cache, &url))
    }

    /// List CacheStorage records for the page origin.
    pub fn cache_records(&self) -> Result<Vec<CacheRecord>, String> {
        let origin = origin::service_worker_origin(&self.session.url);
        Ok(self
            .service_worker_state()?
            .borrow()
            .service_workers
            .cache_records()
            .into_iter()
            .filter(|record| record.origin == origin)
            .collect())
    }
}
