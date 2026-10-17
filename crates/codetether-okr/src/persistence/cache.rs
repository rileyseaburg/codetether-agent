//! Repository cache operations.

use super::OkrRepository;

impl OkrRepository {
    // ============ Utility Methods ============

    /// Clear the in-memory cache (useful for forcing reload from disk)
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// repo.clear_cache().await;
    /// assert!(repo.list_okrs().await.is_ok());
    /// # });
    /// ```
    #[allow(dead_code)]
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.okrs.clear();
        cache.runs.clear();
        tracing::debug!("OKR repository cache cleared");
    }
}
