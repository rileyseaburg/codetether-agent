use super::*;
impl ResultStore {
    /// Query results by tags
    pub async fn query_by_tags(&self, tags: &[String]) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results
            .values()
            .filter(|r| tags.iter().any(|t| r.tags.contains(t)))
            .cloned()
            .collect()
    }

    /// Query results by producer
    pub async fn query_by_producer(&self, producer_id: &str) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results
            .values()
            .filter(|r| r.producer_id == producer_id)
            .cloned()
            .collect()
    }

    /// Query results by key prefix
    pub async fn query_by_prefix(&self, prefix: &str) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results
            .values()
            .filter(|r| r.key.starts_with(prefix))
            .cloned()
            .collect()
    }
}
