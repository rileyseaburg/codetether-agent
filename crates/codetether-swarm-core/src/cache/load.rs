use super::*;
impl SwarmCache {
    /// Load index from disk
    async fn load_index(&mut self) -> Result<()> {
        let ttl = Duration::from_secs(self.config.ttl_secs);

        let mut entries = match fs::read_dir(&self.cache_dir).await {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json")
                && let Some(key) = path.file_stem().and_then(|s| s.to_str())
            {
                match fs::read_to_string(&path).await {
                    Ok(json) => {
                        if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&json) {
                            if !cache_entry.is_expired(ttl) {
                                self.index.insert(key.to_string(), cache_entry);
                            } else {
                                self.stats.expired_removed += 1;
                                let _ = fs::remove_file(&path).await;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!(path = %path.display(), error = %e, "Failed to read cache entry");
                    }
                }
            }
        }

        self.stats.current_entries = self.index.len();
        Ok(())
    }
}
