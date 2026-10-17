//! Initialize memory identity, timestamps, importance and access state.
use super::MemoryEntry;
use chrono::Utc;
impl MemoryEntry {
    /// Create a record with a fresh ID, current timestamps and medium importance.
    ///
    /// # Arguments
    /// * `content` - Memory text.
    /// * `tags` - Search and grouping labels.
    /// # Returns
    /// An unaccessed record with no cached embedding.
    /// # Examples
    /// ```rust
    /// let entry = codetether_memory_core::MemoryEntry::new("remember this", vec![]);
    /// assert_eq!(entry.access_count, 0);
    /// assert!(entry.embedding.is_none());
    /// ```
    pub fn new(content: impl Into<String>, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.into(),
            tags,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            scope: None,
            source: None,
            importance: 3, // default medium importance
            embedding: None,
        }
    }
}
