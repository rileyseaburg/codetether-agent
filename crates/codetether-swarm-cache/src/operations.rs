use super::*;
impl SwarmCache {
    /// Generate a task key using the standard library hasher.
    ///
    /// # Arguments
    /// * `task` - Task identity, instruction, and execution context.
    ///
    /// # Returns
    /// A hexadecimal key for the task and its dependency results.
    ///
    /// # Examples
    /// ```
    /// let task = codetether_swarm_core::SubTask::new("example", "inspect source");
    /// assert_eq!(codetether_swarm_core::cache::SwarmCache::generate_key(&task).len(), 16);
    /// ```
    pub fn generate_key(task: &SubTask) -> String {
        codetether_swarm_core::cache::SwarmCache::generate_key(task)
    }
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
        self.inner.get(task).await
    }
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
        self.inner.put(task, result).await
    }
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
        self.inner.clear().await
    }
}
