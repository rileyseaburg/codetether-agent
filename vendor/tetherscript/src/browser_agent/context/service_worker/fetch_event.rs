//! Service-worker fetch interception metadata.

/// Deterministic record of a fetch observed by an active service worker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceWorkerFetchEvent {
    /// Monotonic sequence within the owning context.
    pub sequence: u64,
    /// Origin that owns the controlling registration.
    pub origin: String,
    /// Registration scope that controlled the request.
    pub scope: String,
    /// Absolute request URL observed by the worker.
    pub request_url: String,
    /// Cache bucket that supplied the response, when any.
    pub cache_name: Option<String>,
    /// Whether CacheStorage supplied a response.
    pub matched: bool,
    /// Response status returned from CacheStorage, when matched.
    pub status: Option<u16>,
}

impl ServiceWorkerFetchEvent {
    pub(crate) fn new(sequence: u64, origin: &str, scope: &str, url: &str) -> Self {
        Self {
            sequence,
            origin: origin.into(),
            scope: scope.into(),
            request_url: url.into(),
            cache_name: None,
            matched: false,
            status: None,
        }
    }
}
