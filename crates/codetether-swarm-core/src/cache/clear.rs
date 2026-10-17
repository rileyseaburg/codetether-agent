use super::*;
impl SwarmCache {
    /// Remove all entries from the index and JSON files from the cache directory.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Success after traversing the directory; individual removal failures are ignored.
    ///
    /// # Errors
    /// Returns an error if the cache directory cannot be opened or traversed.
    ///
    /// # Examples
    /// See [SwarmCache] for construction; call `cache.clear().await?`.
    pub async fn clear(&mut self) -> Result<()> {
        self.index.clear();
        self.stats.current_entries = 0;

        // Remove all files in cache directory
        let mut entries = fs::read_dir(&self.cache_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                let _ = fs::remove_file(&path).await;
            }
        }

        tracing::info!("Cache cleared");
        Ok(())
    }
}
