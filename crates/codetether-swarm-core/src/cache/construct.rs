use super::*;
impl SwarmCache {
    /// Create a cache, resolving the fallback directory only when needed.
    ///
    /// # Arguments
    /// * `config` - Cache limits, enabled state, and optional explicit directory.
    /// * `default_cache_dir` - Host-provided directory used when none is configured.
    ///
    /// # Returns
    /// A cache populated from unexpired entries on disk.
    ///
    /// # Errors
    /// Returns an error if directory creation or index traversal fails.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let cache = codetether_swarm_core::cache::SwarmCache::new(
    ///     Default::default(), || std::path::PathBuf::from("cache"),
    /// ).await?;
    /// assert_eq!(cache.cache_dir(), std::path::Path::new("cache"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(
        config: CacheConfig,
        default_cache_dir: impl FnOnce() -> PathBuf,
    ) -> Result<Self> {
        let cache_dir = config.cache_dir.clone().unwrap_or_else(default_cache_dir);

        // Ensure cache directory exists
        fs::create_dir_all(&cache_dir).await?;

        let mut cache = Self {
            config,
            cache_dir,
            stats: CacheStats::default(),
            index: HashMap::new(),
        };

        // Load existing index
        cache.load_index().await?;

        tracing::info!(
            cache_dir = %cache.cache_dir.display(),
            entries = cache.index.len(),
            "Swarm cache initialized"
        );

        Ok(cache)
    }
}
