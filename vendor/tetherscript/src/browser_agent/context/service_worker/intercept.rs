//! Service-worker fetch interception over CacheStorage.

use super::origin::{service_worker_origin, service_worker_url};
use super::{CacheResponse, ServiceWorkerStore};

impl ServiceWorkerStore {
    pub(crate) fn intercept_fetch(
        &mut self,
        page_url: &str,
        request_url: &str,
    ) -> Option<CacheResponse> {
        let origin = service_worker_origin(page_url);
        let request_url = service_worker_url(&origin, request_url);
        let registration = self.active_for(&origin, &request_url)?;
        let cached = self.caches.first_match(&origin, &request_url);
        let hit = cached
            .as_ref()
            .map(|(cache, response)| (cache.clone(), response.status));
        self.push_fetch(&origin, &registration.scope, &request_url, hit);
        cached.map(|(_, response)| response)
    }
}
