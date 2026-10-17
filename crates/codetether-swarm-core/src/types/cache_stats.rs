//! CacheStats shared contract.
use super::*;

/// Cache statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let stats = codetether_swarm_core::CacheStats::default();
/// assert_eq!(stats.total_lookups(), 0);
/// ```
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of entries evicted due to size limits
    pub evictions: u64,
    /// Number of expired entries removed
    pub expired_removed: u64,
    /// Current number of entries in cache
    pub current_entries: usize,
}
