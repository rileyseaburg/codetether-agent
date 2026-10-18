//! Cache inspection and invalidation APIs.

use super::SwarmExecutor;
use crate::swarm::CacheStats;
use anyhow::Result;

impl SwarmExecutor {
    /// Returns cache statistics when caching is enabled.
    pub async fn cache_stats(&self) -> Option<CacheStats> {
        self.inner.cache_stats().await
    }

    /// Clears all cached subtask results.
    ///
    /// # Errors
    ///
    /// Returns an error when the cache backend cannot be cleared.
    pub async fn clear_cache(&self) -> Result<()> {
        self.inner.clear_cache().await
    }
}
