//! Service-worker store CacheStorage operations.

use super::{CacheRecord, CacheResponse, ServiceWorkerStore};

impl ServiceWorkerStore {
    /// Insert or replace one cached response.
    pub fn cache_put(&mut self, origin: &str, cache: &str, url: &str, response: CacheResponse) {
        self.caches.put(origin, cache, url, response);
    }

    /// Match one cached response.
    pub fn cache_match(&self, origin: &str, cache: &str, url: &str) -> Option<CacheResponse> {
        self.caches.match_request(origin, cache, url)
    }

    /// Delete one cached response.
    pub fn cache_delete(&mut self, origin: &str, cache: &str, url: &str) -> bool {
        self.caches.delete(origin, cache, url)
    }

    /// List request keys for one cache bucket.
    pub fn cache_keys(&self, origin: &str, cache: &str) -> Vec<String> {
        self.caches.keys(origin, cache)
    }

    /// List all cache records.
    pub fn cache_records(&self) -> Vec<CacheRecord> {
        self.caches.records()
    }
}
