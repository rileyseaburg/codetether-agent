//! knowledge graph stats for codebase indexing.

use super::*;

/// Counts of the node categories and relationships in a knowledge graph.
///
/// # Examples
///
/// ```
/// let stats = codetether_indexer::KnowledgeGraphStats::default();
/// assert_eq!(stats.edges, 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeGraphStats {
    pub file_nodes: u64,
    pub symbol_nodes: u64,
    pub module_nodes: u64,
    pub symbol_reference_nodes: u64,
    pub edges: u64,
}
