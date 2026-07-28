//! Static route table used by static HTTP workers.

use std::collections::HashMap;

use super::cache::CachedResponse;

/// Fully prebuilt static site responses.
pub(crate) struct Site {
    pub(crate) routes: HashMap<String, CachedResponse>,
    pub not_found: CachedResponse,
    pub method_not_allowed: CachedResponse,
    pub bad_request: CachedResponse,
}

impl Site {
    /// Find the cached response for a request path.
    pub(crate) fn route(&self, path: &str) -> Option<&CachedResponse> {
        self.routes.get(path)
    }
}
