//! codebase index for codebase indexing.

use super::*;

/// A persisted workspace index, including file vectors and source relationships.
///
/// # Examples
///
/// ```
/// let index = codetether_indexer::CodebaseIndex {
///     version: 3, root: ".".into(), generated_at: chrono::Utc::now(),
///     embedding_provider: "disabled".into(), embedding_model: "disabled".into(),
///     stats: Default::default(), files: Vec::new(),
///     knowledge_graph: codetether_indexer::KnowledgeGraph {
///         version: 1, nodes: Vec::new(), edges: Vec::new(), stats: Default::default(),
///     },
/// };
/// assert!(index.files.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseIndex {
    pub version: u32,
    pub root: String,
    pub generated_at: DateTime<Utc>,
    pub embedding_provider: String,
    pub embedding_model: String,
    pub stats: IndexStats,
    pub files: Vec<IndexedFile>,
    pub knowledge_graph: KnowledgeGraph,
}
