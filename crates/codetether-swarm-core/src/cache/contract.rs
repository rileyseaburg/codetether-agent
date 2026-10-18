//! Cache inspection and invalidation contract for executor state.
use super::SwarmCache;
use crate::CacheStats;
use anyhow::Result;
use std::future::Future;

/// Cache operations needed by executor management APIs.
/// # Implementors
/// The shared cache and application-directory cache implement this contract.
/// Executors retain synchronization and optional-cache handling.
pub trait CacheAccess: Send {
    /// Borrow the current statistics without resetting counters.
    fn stats(&self) -> &CacheStats;
    /// Clear all stored entries using the cache's existing storage policy.
    /// # Errors
    /// Returns storage traversal or clearing errors.
    fn clear(&mut self) -> impl Future<Output = Result<()>> + Send;
}

impl CacheAccess for SwarmCache {
    fn stats(&self) -> &CacheStats {
        SwarmCache::stats(self)
    }
    async fn clear(&mut self) -> Result<()> {
        SwarmCache::clear(self).await
    }
}
