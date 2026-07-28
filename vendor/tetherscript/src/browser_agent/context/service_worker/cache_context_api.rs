//! Browser context CacheStorage APIs.

use super::{origin, CacheRecord, CacheResponse};
use crate::browser_agent::context::BrowserContext;

impl BrowserContext {
    /// Insert or replace one origin-scoped cache response.
    pub fn cache_put(
        &mut self,
        origin_or_url: &str,
        cache: &str,
        request_url: &str,
        response: CacheResponse,
    ) {
        let origin = origin::service_worker_origin(origin_or_url);
        let url = origin::service_worker_url(&origin, request_url);
        self.state
            .borrow_mut()
            .service_workers
            .cache_put(&origin, cache, &url, response);
    }

    /// Match one origin-scoped cache response.
    pub fn cache_match(
        &self,
        origin_or_url: &str,
        cache: &str,
        request_url: &str,
    ) -> Option<CacheResponse> {
        let origin = origin::service_worker_origin(origin_or_url);
        let url = origin::service_worker_url(&origin, request_url);
        self.state
            .borrow()
            .service_workers
            .cache_match(&origin, cache, &url)
    }

    /// Delete one origin-scoped cache response.
    pub fn cache_delete(&mut self, origin_or_url: &str, cache: &str, request_url: &str) -> bool {
        let origin = origin::service_worker_origin(origin_or_url);
        let url = origin::service_worker_url(&origin, request_url);
        self.state
            .borrow_mut()
            .service_workers
            .cache_delete(&origin, cache, &url)
    }

    /// List request URLs for one origin-scoped cache.
    pub fn cache_keys(&self, origin_or_url: &str, cache: &str) -> Vec<String> {
        let origin = origin::service_worker_origin(origin_or_url);
        self.state
            .borrow()
            .service_workers
            .cache_keys(&origin, cache)
    }

    /// List all CacheStorage records in deterministic order.
    pub fn cache_records(&self) -> Vec<CacheRecord> {
        self.state.borrow().service_workers.cache_records()
    }
}
