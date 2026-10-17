//! indexed file for codebase indexing.

use super::*;

/// Metadata and an optional embedding for one workspace-relative source file.
///
/// # Examples
///
/// ```
/// let file = codetether_indexer::IndexedFile {
///     path: "src/lib.rs".into(), language: "rust".into(), bytes: 0,
///     lines: 0, symbol_hints: 0, modified_unix_ms: None, embedding: Vec::new(),
/// };
/// assert!(file.embedding.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub path: String,
    pub language: String,
    pub bytes: u64,
    pub lines: u32,
    pub symbol_hints: u32,
    pub modified_unix_ms: Option<i64>,
    pub embedding: Vec<f32>,
}
