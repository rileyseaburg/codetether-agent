//! Bridge connection and receipt counters.
use super::*;
impl BusBridge {
    /// Current bridge status summary (JSON-friendly).
    ///
    /// # Arguments
    /// Borrows the bridge.
    ///
    /// # Returns
    /// A snapshot of connection status, receipts and capacity.
    ///
    /// # Examples
    /// ```rust
    /// let bridge = codetether_bus_http::BusBridge::new("http://localhost/v1/bus/stream".into());
    /// assert_eq!(bridge.status().total_received, 0);
    /// ```
    pub fn status(&self) -> BusBridgeStatus {
        BusBridgeStatus {
            connected: self.connected.load(Ordering::Relaxed),
            total_received: self.total_received.load(Ordering::Relaxed),
            bus_url: self.bus_url.clone(),
            buffer_capacity: self.capacity,
        }
    }
}
