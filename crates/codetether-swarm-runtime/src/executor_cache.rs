//! Synchronized cache inspection and invalidation for executor state.
use crate::ExecutorState;
use anyhow::Result;
use codetether_swarm_core::{CacheStats, cache::CacheAccess};

impl<A, C: CacheAccess> ExecutorState<A, C> {
    /// Snapshot cache statistics when a cache is attached.
    /// # Returns
    /// Cloned statistics under the existing cache mutex, or `None` without a cache.
    /// # Examples
    /// ```no_run
    /// # async fn example<A, C: codetether_swarm_core::cache::CacheAccess>(state: &codetether_swarm_runtime::ExecutorState<A, C>) {
    /// let _stats = state.cache_stats().await;
    /// # }
    /// ```
    pub async fn cache_stats(&self) -> Option<CacheStats> {
        let cache = self.cache.as_ref()?;
        Some(cache.lock().await.stats().clone())
    }

    /// Clear the attached cache, or return immediately if no cache is attached.
    /// # Returns
    /// Unit after clearing while holding the existing mutex.
    /// # Errors
    /// Returns the cache backend's storage error.
    /// # Examples
    /// ```no_run
    /// # async fn example<A, C: codetether_swarm_core::cache::CacheAccess>(state: &codetether_swarm_runtime::ExecutorState<A, C>) -> anyhow::Result<()> {
    /// state.clear_cache().await?;
    /// # Ok(()) }
    /// ```
    pub async fn clear_cache(&self) -> Result<()> {
        if let Some(cache) = &self.cache {
            cache.lock().await.clear().await?;
        }
        Ok(())
    }
}
