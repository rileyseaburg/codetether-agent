//! CacheStorage put and match operations.

use super::{CacheRecord, CacheResponse, CacheStore};

impl CacheStore {
    /// Insert or replace one cache response.
    pub fn put(&mut self, origin: &str, cache: &str, url: &str, response: CacheResponse) {
        if let Some(record) = self.find_mut(origin, cache, url) {
            record.response = response;
            return;
        }
        self.records
            .push(CacheRecord::new(origin, cache, url, response));
    }

    /// Match one cache response by exact request URL.
    pub fn match_request(&self, origin: &str, cache: &str, url: &str) -> Option<CacheResponse> {
        self.records
            .iter()
            .find(|record| same_key(record, origin, cache, url))
            .map(|record| record.response.clone())
    }

    fn find_mut(&mut self, origin: &str, cache: &str, url: &str) -> Option<&mut CacheRecord> {
        self.records
            .iter_mut()
            .find(|record| same_key(record, origin, cache, url))
    }
}

fn same_key(record: &CacheRecord, origin: &str, cache: &str, url: &str) -> bool {
    record.origin == origin && record.cache_name == cache && record.request_url == url
}
