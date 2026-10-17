use super::*;
impl SwarmCache {
    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get mutable reference to stats
    pub fn stats_mut(&mut self) -> &mut CacheStats {
        &mut self.stats
    }

    /// Get the cache directory path.
    pub fn cache_dir(&self) -> &std::path::Path {
        &self.cache_dir
    }

    /// Set bypass mode for current execution
    pub fn set_bypass(&mut self, bypass: bool) {
        self.config.bypass = bypass;
    }
}
