//! Service-worker fetch log helpers.

use super::{ServiceWorkerFetchEvent, ServiceWorkerStore};

impl ServiceWorkerStore {
    /// Return deterministic fetch interception records.
    pub fn fetch_log(&self) -> Vec<ServiceWorkerFetchEvent> {
        self.fetch_log.clone()
    }

    pub(crate) fn push_fetch(
        &mut self,
        origin: &str,
        scope: &str,
        url: &str,
        hit: Option<(String, u16)>,
    ) {
        let mut event = ServiceWorkerFetchEvent::new(self.next_sequence, origin, scope, url);
        if let Some((cache, status)) = hit {
            event.cache_name = Some(cache);
            event.matched = true;
            event.status = Some(status);
        }
        self.next_sequence += 1;
        self.fetch_log.push(event);
    }
}
