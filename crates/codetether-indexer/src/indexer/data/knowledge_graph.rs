//! knowledge graph for codebase indexing.

use super::*;

/// Source files, definitions, and imports represented as a directed graph.
///
/// # Examples
///
/// ```
/// let graph = codetether_indexer::KnowledgeGraph {
///     version: 1, nodes: Vec::new(), edges: Vec::new(), stats: Default::default(),
/// };
/// assert!(graph.nodes.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub version: u32,
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
    pub stats: KnowledgeGraphStats,
}
