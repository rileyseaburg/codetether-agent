use super::*;
impl SwarmCache {
    /// Get accumulated cache statistics.
    pub fn stats(&self) -> &CacheStats {
        self.inner.stats()
    }
    /// Get mutable cache statistics.
    pub fn stats_mut(&mut self) -> &mut CacheStats {
        self.inner.stats_mut()
    }
    /// Get the resolved cache directory.
    pub fn cache_dir(&self) -> &std::path::Path {
        self.inner.cache_dir()
    }
    /// Enable or disable bypass for subsequent operations.
    pub fn set_bypass(&mut self, bypass: bool) {
        self.inner.set_bypass(bypass);
    }
}
