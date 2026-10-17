//! Launch background ingestion for a bridge.
use super::*;
impl BusBridge {
    /// Spawn the SSE reader as a background tokio task.
    ///
    /// Returns `Self` wrapped in an `Arc` for sharing with tool handlers.
    ///
    /// # Arguments
    /// Consumes the configured bridge.
    ///
    /// # Returns
    /// A shared handle after starting its background ingestion task.
    ///
    /// # Examples
    /// ```rust,no_run
    /// let bridge = codetether_bus_http::BusBridge::new("http://localhost/v1/bus/stream".into());
    /// let shared = bridge.spawn();
    /// assert!(!shared.status().bus_url.is_empty());
    /// ```
    pub fn spawn(self) -> Arc<Self> {
        let bridge = Arc::new(self);
        let bg = Arc::clone(&bridge);
        tokio::spawn(async move {
            bg.reader_loop().await;
        });
        bridge
    }
}
