use super::*;
impl SwarmCache {
    /// Enforce size limits by evicting oldest entries
    async fn enforce_size_limits(&mut self) -> Result<()> {
        if self.index.len() < self.config.max_entries {
            return Ok(());
        }

        // Sort by creation time and remove oldest
        let mut entries: Vec<_> = self
            .index
            .iter()
            .map(|(k, v)| (k.clone(), v.created_at))
            .collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        let to_remove = self.index.len() - self.config.max_entries + 1;
        for (key, _) in entries.into_iter().take(to_remove) {
            self.index.remove(&key);
            let _ = self.remove_from_disk(&key);
            self.stats.evictions += 1;
        }

        self.stats.current_entries = self.index.len();

        Ok(())
    }
}
