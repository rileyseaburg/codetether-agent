use super::*;
impl SwarmCache {
    /// Get the path for a cache entry file
    fn entry_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", key))
    }

    /// Save an entry to disk
    async fn save_to_disk(&self, key: &str, entry: &CacheEntry) -> Result<()> {
        let path = self.entry_path(key);
        let json = serde_json::to_string_pretty(entry)?;
        fs::write(&path, json).await?;
        Ok(())
    }

    /// Remove an entry from disk
    async fn remove_from_disk(&self, key: &str) -> Result<()> {
        let path = self.entry_path(key);
        if path.exists() {
            fs::remove_file(&path).await?;
        }
        Ok(())
    }
}
