//! CacheStorage record and response types.

/// Static response stored in deterministic CacheStorage.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheResponse {
    /// HTTP status code returned to page fetch callers.
    pub status: u16,
    /// Stored response headers.
    pub headers: Vec<(String, String)>,
    /// Stored text body.
    pub body: String,
}

impl CacheResponse {
    /// Build a text response without headers.
    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: body.into(),
        }
    }
}

/// One origin-scoped CacheStorage entry.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheRecord {
    /// Origin key such as `https://example.test`.
    pub origin: String,
    /// Cache bucket name.
    pub cache_name: String,
    /// Absolute request URL key.
    pub request_url: String,
    /// Stored synthetic response.
    pub response: CacheResponse,
}

impl CacheRecord {
    pub(crate) fn new(origin: &str, cache: &str, url: &str, response: CacheResponse) -> Self {
        Self {
            origin: origin.into(),
            cache_name: cache.into(),
            request_url: url.into(),
            response,
        }
    }
}
