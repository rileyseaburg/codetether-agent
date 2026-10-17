use super::*;
impl ResultStore {
    /// Get a result by key, including expired entries until explicit cleanup.
    pub async fn get(&self, key: &str) -> Option<SharedResult> {
        let results = self.results.read().await;
        results.get(key).cloned()
    }

    /// Get a result and deserialize it to a specific type.
    ///
    /// # Arguments
    /// * `key` - Key previously used for publication.
    ///
    /// # Returns
    /// The stored JSON value decoded as T.
    ///
    /// # Errors
    /// Returns an error when the key is absent or the value cannot decode as T.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example(store: &codetether_swarm_core::result_store::ResultStore) -> anyhow::Result<()> {
    /// let value: String = store.get_typed("key").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_typed<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let result = self
            .get(key)
            .await
            .ok_or_else(|| anyhow!("Result not found: {}", key))?;

        serde_json::from_value(result.value)
            .map_err(|e| anyhow!("Failed to deserialize result: {}", e))
    }

    /// Get all results (for debugging/inspection)
    pub async fn get_all(&self) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results.values().cloned().collect()
    }
}
