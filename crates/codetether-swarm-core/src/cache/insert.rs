use super::*;
impl SwarmCache {
    /// Persist a successful task result when caching is enabled.
    ///
    /// # Arguments
    /// * `task` - Task that produced the result.
    /// * `result` - Successful execution result to retain.
    ///
    /// # Returns
    /// Success after caching, or immediately when caching is bypassed or the result failed.
    ///
    /// # Errors
    /// Returns an error if enforcing limits or writing the entry fails.
    ///
    /// # Examples
    /// See [SwarmCache] for construction; call `cache.put(&task, &result).await?`.
    pub async fn put(&mut self, task: &SubTask, result: &SubTaskResult) -> Result<()> {
        if !self.config.enabled || self.config.bypass {
            return Ok(());
        }

        // Only cache successful results
        if !result.success {
            tracing::debug!(task_id = %task.id, "Not caching failed result");
            return Ok(());
        }

        // Check if we need to evict entries
        self.enforce_size_limits().await?;

        let key = Self::generate_key(task);
        let content_hash = Self::generate_content_hash(task);

        let entry = CacheEntry {
            result: result.clone(),
            created_at: SystemTime::now(),
            content_hash,
            task_name: task.name.clone(),
        };

        // Store on disk
        self.save_to_disk(&key, &entry).await?;

        // Update in-memory index
        self.index.insert(key.clone(), entry);
        self.stats.current_entries = self.index.len();

        tracing::info!(key = %key, task_name = %task.name, "Cached result");

        Ok(())
    }
}
