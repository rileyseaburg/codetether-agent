//! Read recent buffered events in chronological order.
use super::*;
impl BusBridge {
    /// Query recent events with optional topic filter and limit.
    ///
    /// # Arguments
    /// `topic_filter` and `since` restrict events; `limit` bounds the newest matching events.
    ///
    /// # Returns
    /// Owned matching envelopes in chronological order.
    ///
    /// # Examples
    /// ```rust
    /// async fn inspect(bridge: &codetether_bus_http::BusBridge) {
    ///     let events = bridge.recent_events(None, 10, None).await;
    ///     assert!(events.len() <= 10);
    /// }
    /// ```
    pub async fn recent_events(
        &self,
        topic_filter: Option<&str>,
        limit: usize,
        since: Option<DateTime<Utc>>,
    ) -> Vec<BusEnvelope> {
        let buf = self.buffer.read().await;
        buf.iter()
            .rev()
            .filter(|env| {
                if let Some(filter) = topic_filter {
                    topic_matches(&env.topic, filter)
                } else {
                    true
                }
            })
            .filter(|env| {
                if let Some(ts) = since {
                    env.timestamp >= ts
                } else {
                    true
                }
            })
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev() // restore chronological order
            .collect()
    }

    /// Buffer size (number of envelopes currently held).
    ///
    /// # Arguments
    /// Borrows the bridge.
    ///
    /// # Returns
    /// The current number of buffered envelopes.
    ///
    /// # Examples
    /// ```rust
    /// async fn inspect(bridge: &codetether_bus_http::BusBridge) {
    ///     let count = bridge.buffer_len().await;
    ///     assert!(count <= bridge.status().buffer_capacity);
    /// }
    /// ```
    pub async fn buffer_len(&self) -> usize {
        self.buffer.read().await.len()
    }
}
