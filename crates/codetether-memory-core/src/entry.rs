//! Persisted memory record with optional cached embedding.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A persisted memory record with metadata and an optional cached vector.
///
/// # Examples
/// ```rust
/// let entry = codetether_memory_core::MemoryEntry::new("bounded output", vec!["tools".into()]);
/// assert_eq!(entry.importance, 3);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique identifier
    pub id: String,
    /// The memory content
    pub content: String,
    /// Tags for categorization/search
    pub tags: Vec<String>,
    /// When this memory was created
    pub created_at: DateTime<Utc>,
    /// When this memory was last accessed
    pub accessed_at: DateTime<Utc>,
    /// How many times this memory has been accessed
    pub access_count: u64,
    /// Optional project/context scope
    pub scope: Option<String>,
    /// Source of the memory (session_id, tool, etc.)
    pub source: Option<String>,
    /// Importance level (1-5)
    pub importance: u8,
    /// Cached semantic embedding (computed once on save, persisted to disk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedding: Option<codetether_vectordb::EmbeddingVector>,
}
