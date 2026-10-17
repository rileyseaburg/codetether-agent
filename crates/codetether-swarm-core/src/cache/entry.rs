use super::*;

/// Cache entry storing a subtask result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```no_run
/// # fn example(result: codetether_swarm_core::SubTaskResult) {
/// let entry = codetether_swarm_core::cache::CacheEntry { result, created_at: std::time::SystemTime::now(), content_hash: String::new(), task_name: String::new() };
/// assert!(!entry.is_expired(std::time::Duration::from_secs(60)));
/// # }
/// ```
pub struct CacheEntry {
    /// The cached result
    pub result: SubTaskResult,
    /// When this entry was created
    pub created_at: SystemTime,
    /// Hash of the task content (for verification)
    pub content_hash: String,
    /// Task name for debugging
    pub task_name: String,
}
impl CacheEntry {
    /// Check if this entry has expired
    pub fn is_expired(&self, ttl: Duration) -> bool {
        match self.created_at.elapsed() {
            Ok(elapsed) => elapsed > ttl,
            Err(_) => {
                // Clock went backwards, treat as expired
                true
            }
        }
    }
}
