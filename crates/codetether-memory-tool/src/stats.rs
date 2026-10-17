//! Summary of stored memories and their tag usage.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Counts and tag frequencies from a memory store.
///
/// # Examples
/// ```rust
/// let stats = codetether_memory_tool::MemoryStats { total_entries: 0, total_accesses: 0,
///     unique_tags: 0, tags: std::collections::HashMap::new() };
/// assert_eq!(stats.total_entries, 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: usize,
    pub total_accesses: u64,
    pub unique_tags: usize,
    pub tags: HashMap<String, u64>,
}
