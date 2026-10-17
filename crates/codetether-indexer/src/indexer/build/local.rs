//! Local default entry point for standalone index construction.

use super::*;

/// Build an index with local embeddings or embeddings disabled.
///
/// # Arguments
/// * `root` - Workspace directory to scan.
/// * `options` - File filtering and local embedding options.
/// # Returns
/// A sorted index and its workspace knowledge graph.
/// # Errors
/// Returns filesystem or indexing errors; remote embeddings require a resolver.
/// # Examples
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let index = codetether_indexer::build_index(std::path::Path::new("."), &Default::default()).await.unwrap();
/// assert!(!index.root.is_empty());
/// # });
/// ```
pub async fn build_index(root: &Path, options: &BuildOptions) -> Result<CodebaseIndex> {
    build_index_with_resolver(root, options, &crate::local_resolver::LocalResolver).await
}
