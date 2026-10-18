use super::*;
impl SwarmCache {
    /// Look up an unexpired matching task result and update statistics.
    ///
    /// # Arguments
    /// * `task` - Task whose cached result is requested.
    ///
    /// # Returns
    /// The cached result, or None when disabled, bypassed, absent, or expired.
    ///
    /// # Examples
    /// See [SwarmCache] for construction; call `cache.get(&task).await`.
    pub async fn get(&mut self, task: &SubTask) -> Option<SubTaskResult> {
        if !self.config.enabled || self.config.bypass {
            return None;
        }

        let key = Self::generate_key(task);

        // Check in-memory index first
        if let Some(entry) = self.index.get(&key) {
            let ttl = Duration::from_secs(self.config.ttl_secs);

            if entry.is_expired(ttl) {
                tracing::debug!(key = %key, "Cache entry expired");
                self.stats.expired_removed += 1;
                self.index.remove(&key);
                let _ = self.remove_from_disk(&key).await;
                self.stats.record_miss();
                return None;
            }

            // Verify content hash matches
            let current_hash = Self::generate_content_hash(task);
            if entry.content_hash != current_hash {
                tracing::debug!(key = %key, "Content hash mismatch, cache invalid");
                self.index.remove(&key);
                let _ = self.remove_from_disk(&key).await;
                self.stats.record_miss();
                return None;
            }

            tracing::info!(key = %key, task_name = %entry.task_name, "Cache hit");
            self.stats.record_hit();
            return Some(entry.result.clone());
        }

        self.stats.record_miss();
        None
    }
}
