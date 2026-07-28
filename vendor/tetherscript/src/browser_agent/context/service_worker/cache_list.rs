//! CacheStorage deterministic listing operations.

use super::{CacheRecord, CacheResponse, CacheStore};

impl CacheStore {
    /// List request URL keys for one cache in deterministic order.
    pub fn keys(&self, origin: &str, cache: &str) -> Vec<String> {
        let mut keys: Vec<_> = self
            .records
            .iter()
            .filter(|record| record.origin == origin && record.cache_name == cache)
            .map(|record| record.request_url.clone())
            .collect();
        keys.sort();
        keys
    }

    /// List all cache records in deterministic order.
    pub fn records(&self) -> Vec<CacheRecord> {
        let mut records = self.records.clone();
        records.sort();
        records
    }

    pub(crate) fn first_match(&self, origin: &str, url: &str) -> Option<(String, CacheResponse)> {
        let mut hits: Vec<_> = self
            .records
            .iter()
            .filter(|record| record.origin == origin && record.request_url == url)
            .collect();
        hits.sort_by(|left, right| left.cache_name.cmp(&right.cache_name));
        hits.first()
            .map(|record| (record.cache_name.clone(), record.response.clone()))
    }
}
