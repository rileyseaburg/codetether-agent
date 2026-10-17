use super::*;

/// Configuration for the swarm cache
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let config = codetether_swarm_core::cache::CacheConfig::default();
/// assert!(config.enabled);
/// ```
pub struct CacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Time-to-live for cache entries (seconds)
    pub ttl_secs: u64,
    /// Maximum number of entries in cache
    pub max_entries: usize,
    /// Maximum size of cache directory in MB
    pub max_size_mb: u64,
    /// Cache directory path (None = use default)
    pub cache_dir: Option<PathBuf>,
    /// Whether to bypass cache for this execution
    pub bypass: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_secs: 86400, // 24 hours
            max_entries: 1000,
            max_size_mb: 100,
            cache_dir: None,
            bypass: false,
        }
    }
}
