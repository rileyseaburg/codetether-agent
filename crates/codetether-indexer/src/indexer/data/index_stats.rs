//! index stats for codebase indexing.

use super::*;

/// Counts collected while scanning and embedding a workspace.
///
/// # Examples
///
/// ```
/// let stats = codetether_indexer::IndexStats::default();
/// assert_eq!(stats.indexed_files, 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_seen_files: u64,
    pub indexed_files: u64,
    pub skipped_hidden: u64,
    pub skipped_non_text: u64,
    pub skipped_large: u64,
    pub skipped_io_errors: u64,
    pub total_bytes: u64,
    pub total_lines: u64,
    pub embedded_files: u64,
    pub embedding_dimensions: u32,
    pub embedding_prompt_tokens: u64,
    pub embedding_total_tokens: u64,
    pub language_counts: HashMap<String, u64>,
}
