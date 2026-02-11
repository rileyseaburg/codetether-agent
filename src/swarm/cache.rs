//! Swarm result caching for avoiding duplicate task execution
//!
//! Uses content-based hashing to identify identical tasks and cache
//! their results to disk for reuse across executions.

use super::{SubTask, SubTaskResult};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::fs;
use tracing;

/// Cache entry storing a subtask result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Cache statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of entries evicted due to size limits
    pub evictions: u64,
    /// Number of expired entries removed
    pub expired_removed: u64,
    /// Current number of entries in cache
    pub current_entries: usize,
}

impl CacheStats {
    /// Total number of cache lookups
    pub fn total_lookups(&self) -> u64 {
        self.hits + self.misses
    }

    /// Cache hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_lookups();
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Record a cache hit
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }

    /// Record a cache miss
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }
}

/// Configuration for the swarm cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Time-to-live for cache entries (seconds)
    pub ttl_secs: u64,
    /// Maximum number of entries in cache
    pub max_entries: usize,
    /// Maximum size of cache directory in MB
    pub max_size_mb: u64,
    /// Cache directory path (None = use default)
    pub cache_dir: Option<PathBuf>,
    /// Whether to bypass cache for this execution
    pub bypass: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_secs: 86400, // 24 hours
            max_entries: 1000,
            max_size_mb: 100,
            cache_dir: None,
            bypass: false,
        }
    }
}

/// Swarm result cache using content-based hashing
pub struct SwarmCache {
    config: CacheConfig,
    cache_dir: PathBuf,
    stats: CacheStats,
    /// In-memory index of cache entries
    index: HashMap<String, CacheEntry>,
}

impl SwarmCache {
    /// Create a new cache with the given configuration
    pub async fn new(config: CacheConfig) -> Result<Self> {
        let cache_dir = config
            .cache_dir
            .clone()
            .unwrap_or_else(Self::default_cache_dir);

        // Ensure cache directory exists
        fs::create_dir_all(&cache_dir).await?;

        let mut cache = Self {
            config,
            cache_dir,
            stats: CacheStats::default(),
            index: HashMap::new(),
        };

        // Load existing index
        cache.load_index().await?;

        tracing::info!(
            cache_dir = %cache.cache_dir.display(),
            entries = cache.index.len(),
            "Swarm cache initialized"
        );

        Ok(cache)
    }

    /// Get the default cache directory
    fn default_cache_dir() -> PathBuf {
        directories::ProjectDirs::from("com", "codetether", "agent")
            .map(|dirs| dirs.cache_dir().join("swarm"))
            .unwrap_or_else(|| PathBuf::from(".cache/swarm"))
    }

    /// Generate a cache key from task content using SHA-256
    pub fn generate_key(task: &SubTask) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Create a deterministic hash from task content
        let mut hasher = DefaultHasher::new();
        task.name.hash(&mut hasher);
        task.instruction.hash(&mut hasher);
        task.specialty.hash(&mut hasher);
        task.max_steps.hash(&mut hasher);

        // Include context that affects execution
        if let Some(parent) = &task.context.parent_task {
            parent.hash(&mut hasher);
        }

        // Hash dependency results that are part of the context
        let mut dep_keys: Vec<_> = task.context.dependency_results.keys().collect();
        dep_keys.sort(); // Ensure deterministic ordering
        for key in dep_keys {
            key.hash(&mut hasher);
            task.context.dependency_results[key].hash(&mut hasher);
        }

        format!("{:016x}", hasher.finish())
    }

    /// Get a cached result if available and not expired
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
                let _ = self.remove_from_disk(&key);
                self.stats.record_miss();
                return None;
            }

            // Verify content hash matches
            let current_hash = Self::generate_content_hash(task);
            if entry.content_hash != current_hash {
                tracing::debug!(key = %key, "Content hash mismatch, cache invalid");
                self.index.remove(&key);
                let _ = self.remove_from_disk(&key);
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

    /// Store a result in the cache
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

    /// Generate a content hash for verification
    fn generate_content_hash(task: &SubTask) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        task.instruction.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

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

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get mutable reference to stats
    pub fn stats_mut(&mut self) -> &mut CacheStats {
        &mut self.stats
    }

    /// Clear all cache entries
    pub async fn clear(&mut self) -> Result<()> {
        self.index.clear();
        self.stats.current_entries = 0;

        // Remove all files in cache directory
        let mut entries = fs::read_dir(&self.cache_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                let _ = fs::remove_file(&path).await;
            }
        }

        tracing::info!("Cache cleared");
        Ok(())
    }

    /// Get the path for a cache entry file
    fn entry_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", key))
    }

    /// Get the cache directory path.
    pub fn cache_dir(&self) -> &std::path::Path {
        &self.cache_dir
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

    /// Load index from disk
    async fn load_index(&mut self) -> Result<()> {
        let ttl = Duration::from_secs(self.config.ttl_secs);

        let mut entries = match fs::read_dir(&self.cache_dir).await {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Some(key) = path.file_stem().and_then(|s| s.to_str()) {
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
        }

        self.stats.current_entries = self.index.len();
        Ok(())
    }

    /// Set bypass mode for current execution
    pub fn set_bypass(&mut self, bypass: bool) {
        self.config.bypass = bypass;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_task(name: &str, instruction: &str) -> SubTask {
        SubTask::new(name, instruction)
    }

    fn create_test_result(success: bool) -> SubTaskResult {
        SubTaskResult {
            subtask_id: "test-123".to_string(),
            subagent_id: "agent-123".to_string(),
            success,
            result: "test result".to_string(),
            steps: 5,
            tool_calls: 3,
            execution_time_ms: 1000,
            error: None,
            artifacts: vec![],
        }
    }

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let temp_dir = tempdir().unwrap();
        let config = CacheConfig {
            enabled: true,
            ttl_secs: 3600,
            max_entries: 100,
            max_size_mb: 10,
            cache_dir: Some(temp_dir.path().to_path_buf()),
            bypass: false,
        };

        let mut cache = SwarmCache::new(config).await.unwrap();

        let task = create_test_task("test task", "do something");
        let result = create_test_result(true);

        // Initially should be a miss
        assert!(cache.get(&task).await.is_none());
        assert_eq!(cache.stats().misses, 1);

        // Store the result
        cache.put(&task, &result).await.unwrap();

        // Now should be a hit
        let cached = cache.get(&task).await;
        assert!(cached.is_some());
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cached.unwrap().result, result.result);
    }

    #[tokio::test]
    async fn test_cache_different_tasks() {
        let temp_dir = tempdir().unwrap();
        let config = CacheConfig {
            enabled: true,
            ttl_secs: 3600,
            max_entries: 100,
            max_size_mb: 10,
            cache_dir: Some(temp_dir.path().to_path_buf()),
            bypass: false,
        };

        let mut cache = SwarmCache::new(config).await.unwrap();

        let task1 = create_test_task("task 1", "do something");
        let task2 = create_test_task("task 2", "do something else");
        let result = create_test_result(true);

        // Store only task1
        cache.put(&task1, &result).await.unwrap();

        // task1 should hit, task2 should miss
        assert!(cache.get(&task1).await.is_some());
        assert!(cache.get(&task2).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_bypass() {
        let temp_dir = tempdir().unwrap();
        let config = CacheConfig {
            enabled: true,
            ttl_secs: 3600,
            max_entries: 100,
            max_size_mb: 10,
            cache_dir: Some(temp_dir.path().to_path_buf()),
            bypass: true, // Bypass enabled
        };

        let mut cache = SwarmCache::new(config).await.unwrap();

        let task = create_test_task("test", "instruction");
        let result = create_test_result(true);

        // Store the result
        cache.put(&task, &result).await.unwrap();

        // Should still be a miss due to bypass
        assert!(cache.get(&task).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_failed_results_not_cached() {
        let temp_dir = tempdir().unwrap();
        let config = CacheConfig {
            enabled: true,
            ttl_secs: 3600,
            max_entries: 100,
            max_size_mb: 10,
            cache_dir: Some(temp_dir.path().to_path_buf()),
            bypass: false,
        };

        let mut cache = SwarmCache::new(config).await.unwrap();

        let task = create_test_task("test", "instruction");
        let failed_result = create_test_result(false);

        // Try to store failed result
        cache.put(&task, &failed_result).await.unwrap();

        // Should not be cached
        assert!(cache.get(&task).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let temp_dir = tempdir().unwrap();
        let config = CacheConfig {
            enabled: true,
            ttl_secs: 3600,
            max_entries: 100,
            max_size_mb: 10,
            cache_dir: Some(temp_dir.path().to_path_buf()),
            bypass: false,
        };

        let mut cache = SwarmCache::new(config).await.unwrap();

        let task = create_test_task("test", "instruction");
        let result = create_test_result(true);

        cache.put(&task, &result).await.unwrap();
        assert!(cache.get(&task).await.is_some());

        cache.clear().await.unwrap();
        assert!(cache.get(&task).await.is_none());
        assert_eq!(cache.stats().current_entries, 0);
    }
}
