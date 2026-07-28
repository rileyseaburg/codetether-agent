//! CacheStorage deletion operations.

use super::CacheStore;

impl CacheStore {
    /// Delete one cache response.
    pub fn delete(&mut self, origin: &str, cache: &str, url: &str) -> bool {
        let Some(index) = self.records.iter().position(|record| {
            record.origin == origin && record.cache_name == cache && record.request_url == url
        }) else {
            return false;
        };
        self.records.remove(index);
        true
    }
}
