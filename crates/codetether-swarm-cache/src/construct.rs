use super::*;
impl SwarmCache {
    /// Create a cache using the configured or application-default directory.
    ///
    /// # Arguments
    /// * `config` - Cache settings and an optional explicit directory.
    ///
    /// # Returns
    /// A cache with its persisted entries loaded.
    ///
    /// # Errors
    /// Returns an error when creating the directory or loading entries fails.
    ///
    /// # Examples
    /// See the [SwarmCache] construction example.
    pub async fn new(config: CacheConfig) -> Result<Self> {
        let inner =
            codetether_swarm_core::cache::SwarmCache::new(config, Self::default_cache_dir).await?;
        Ok(Self { inner })
    }

    /// Get the default cache directory
    fn default_cache_dir() -> PathBuf {
        codetether_paths::data_dir()
            .map(|dirs| dirs.join("cache").join("swarm"))
            .unwrap_or_else(|| PathBuf::from(".codetether-agent/cache/swarm"))
    }
}
