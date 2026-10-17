use super::*;
impl ResultStore {
    /// Remove expired results
    pub async fn cleanup_expired(&self) -> usize {
        let now = chrono::Utc::now();
        let mut results = self.results.write().await;
        let keys_to_remove: Vec<String> = results
            .values()
            .filter(|r| r.expires_at.map(|exp| exp <= now).unwrap_or(false))
            .map(|r| r.key.clone())
            .collect();

        for key in &keys_to_remove {
            results.remove(key);
        }

        keys_to_remove.len()
    }

    /// Clear all results
    pub async fn clear(&self) {
        let mut results = self.results.write().await;
        results.clear();
    }
}
