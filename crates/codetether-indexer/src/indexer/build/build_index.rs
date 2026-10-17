//! Coordinate scanning, embeddings, and graph assembly.

use super::*;

/// Scan source files, construct relationships, and embed batches through a host resolver.
///
/// # Arguments
/// * `root` - Workspace directory; paths in the index are relative to it.
/// * `options` - Scanning and embedding settings.
/// * `resolver` - Invoked only when remote embeddings are requested.
/// # Returns
/// The sorted index, statistics, and knowledge graph.
/// # Errors
/// Returns provider-resolution, embedding, or inconsistent-vector-dimension errors.
/// # Examples
/// ```rust,no_run
/// # async fn build(resolver: &dyn codetether_indexer::EmbeddingResolver) -> anyhow::Result<()> {
/// let index = codetether_indexer::build_index_with_resolver(
///     std::path::Path::new("."), &Default::default(), resolver,
/// ).await?;
/// assert!(!index.root.is_empty());
/// # Ok(()) }
/// ```
pub async fn build_index_with_resolver(
    root: &Path,
    options: &BuildOptions,
    resolver: &dyn EmbeddingResolver,
) -> Result<CodebaseIndex> {
    let mut data = scan_workspace(root, options);
    let (embedding_provider, embedding_model) =
        populate_embeddings(&mut data, options, resolver).await?;
    data.files.sort_by(|a, b| a.path.cmp(&b.path));
    let knowledge_graph = build_knowledge_graph(data.knowledge_inputs);
    Ok(CodebaseIndex {
        version: INDEX_VERSION,
        root: root.display().to_string(),
        generated_at: Utc::now(),
        embedding_provider,
        embedding_model,
        stats: data.stats,
        files: data.files,
        knowledge_graph,
    })
}
