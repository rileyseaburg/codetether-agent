//! store query operations.
use super::{AutoEmbedder, MemoryEntry, MemoryStats, Store};
use codetether_memory_core::search;
use std::collections::HashMap;
impl<B: AutoEmbedder> Store<B> {
    /// Search memories by query or tags
    /// Rank matching memory records.
    ///
    /// # Arguments
    /// `query`, `tags` and `scope` select records; `limit` caps returned results.
    /// # Returns
    /// Matching records in retrieval order.
    /// # Examples
    /// ```rust
    /// assert!(codetether_memory_tool::MemoryStore::default().search(None, None, None, 5).is_empty());
    /// ```
    pub fn search(
        &mut self,
        query: Option<&str>,
        tags: Option<&[String]>,
        scope: Option<&str>,
        limit: usize,
    ) -> Vec<MemoryEntry> {
        search::run(&mut self.entries, query, tags, scope, limit)
    }

    /// Get all tags with counts
    /// Count tag occurrences across stored records.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A map from tag to frequency.
    /// # Examples
    /// ```rust
    /// assert!(codetether_memory_tool::MemoryStore::default().all_tags().is_empty());
    /// ```
    pub fn all_tags(&self) -> HashMap<String, u64> {
        let mut tags: HashMap<String, u64> = HashMap::new();
        for entry in self.entries.values() {
            for tag in &entry.tags {
                *tags.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        tags
    }

    /// Get statistics
    /// Summarize records, accesses and tags.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Current store statistics.
    /// # Examples
    /// ```rust
    /// assert_eq!(codetether_memory_tool::MemoryStore::default().stats().total_entries, 0);
    /// ```
    pub fn stats(&self) -> MemoryStats {
        let total = self.entries.len();
        let total_accesses: u64 = self.entries.values().map(|e| e.access_count).sum();
        let tags = self.all_tags();
        MemoryStats {
            total_entries: total,
            total_accesses,
            unique_tags: tags.len(),
            tags,
        }
    }
}
