//! build options for codebase indexing.

use super::*;

/// File filtering, embedding selection, batching, and retry settings.
///
/// # Examples
///
/// ```
/// let options = codetether_indexer::BuildOptions::default();
/// assert_eq!(options.embedding_dimensions, 384);
/// assert!(!options.include_hidden);
/// ```
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub include_hidden: bool,
    pub include_embeddings: bool,
    pub max_file_size_bytes: u64,
    pub embedding_provider: String,
    pub embedding_model: String,
    pub embedding_dimensions: usize,
    pub embedding_batch_size: usize,
    pub embedding_input_chars: usize,
    pub embedding_max_retries: u32,
    pub embedding_retry_initial_ms: u64,
    pub embedding_retry_max_ms: u64,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            include_hidden: false,
            include_embeddings: true,
            max_file_size_bytes: 1024 * 1024,
            embedding_provider: DEFAULT_EMBEDDING_PROVIDER.to_string(),
            embedding_model: DEFAULT_EMBEDDING_MODEL.to_string(),
            embedding_dimensions: DEFAULT_EMBEDDING_DIMENSIONS,
            embedding_batch_size: DEFAULT_EMBEDDING_BATCH_SIZE,
            embedding_input_chars: DEFAULT_EMBEDDING_INPUT_CHARS,
            embedding_max_retries: DEFAULT_EMBEDDING_MAX_RETRIES,
            embedding_retry_initial_ms: DEFAULT_EMBEDDING_RETRY_INITIAL_MS,
            embedding_retry_max_ms: DEFAULT_EMBEDDING_RETRY_MAX_MS,
        }
    }
}
