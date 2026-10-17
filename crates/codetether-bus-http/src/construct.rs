//! Construct an inactive authenticated bus bridge.
use super::*;
impl BusBridge {
    /// Create a new bridge (does **not** start the background task).
    ///
    /// # Arguments
    /// `bus_url` is the SSE endpoint.
    ///
    /// # Returns
    /// An inactive bridge.
    ///
    /// # Examples
    /// ```rust
    /// let bridge = codetether_bus_http::BusBridge::new("http://localhost/v1/bus/stream".into());
    /// assert!(!bridge.status().connected);
    /// ```
    pub fn new(bus_url: String) -> Self {
        Self::with_auth(bus_url, None)
    }

    /// Create a new bridge with an optional bearer token.
    ///
    /// # Arguments
    /// `bus_url` is the SSE endpoint; `auth_token` is an optional bearer token.
    ///
    /// # Returns
    /// An inactive bridge retaining the provided connection settings.
    ///
    /// # Examples
    /// ```rust
    /// let bridge = codetether_bus_http::BusBridge::with_auth("http://localhost/v1/bus/stream".into(), None);
    /// assert_eq!(bridge.status().total_received, 0);
    /// ```
    pub fn with_auth(bus_url: String, auth_token: Option<String>) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(DEFAULT_BUFFER_SIZE))),
            connected: Arc::new(AtomicBool::new(false)),
            total_received: Arc::new(AtomicU64::new(0)),
            bus_url,
            auth_token,
            capacity: DEFAULT_BUFFER_SIZE,
        }
    }
}
