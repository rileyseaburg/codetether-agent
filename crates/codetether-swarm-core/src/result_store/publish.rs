use super::*;
impl ResultStore {
    /// Publish a result, then broadcast its key, producer and tags.
    ///
    /// # Arguments
    /// * `key` - Key to insert or replace.
    /// * `producer_id` - Publishing task identifier.
    /// * `value` - Serializable value to retain as JSON.
    /// * `tags` - Labels used for queries and notification filtering.
    /// * `expires_at` - Optional deadline applied by explicit expiry cleanup.
    ///
    /// # Returns
    /// A clone of the stored result with an inferred schema and publication time.
    ///
    /// # Errors
    /// Returns an error if value serialization fails; absent listeners are ignored.
    ///
    /// # Examples
    /// See [SharedResult] for publication through an existing store.
    pub async fn publish(
        &self,
        key: impl Into<String>,
        producer_id: impl Into<String>,
        value: impl Serialize,
        tags: Vec<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<SharedResult> {
        let key = key.into();
        let producer_id = producer_id.into();

        let value = serde_json::to_value(value)?;
        let schema = ResultSchema::from_value(&value);

        let result = SharedResult {
            key: key.clone(),
            producer_id: producer_id.clone(),
            value,
            schema,
            published_at: chrono::Utc::now(),
            expires_at,
            tags: tags.clone(),
        };

        // Store the result
        {
            let mut results = self.results.write().await;
            results.insert(key.clone(), result.clone());
        }

        // Notify subscribers
        let notification = ResultNotification {
            key: key.clone(),
            producer_id,
            tags,
        };

        // Broadcast to all listeners (they filter based on their subscription)
        let _ = self.notification_tx.send(notification);

        tracing::info!(key = %key, "Published shared result");

        Ok(result)
    }
}
