//! Borrowed graph lookups without changing storage or edge ordering.
use super::{CodeNode, EdgeKind, KnowledgeGraph};
impl KnowledgeGraph {
    /// Find existing source nodes of call edges targeting a symbol.
    ///
    /// # Arguments
    /// * `node_id` - Identifier of the called symbol.
    ///
    /// # Returns
    /// Callers in edge order, preserving duplicates and omitting missing nodes.
    ///
    /// # Examples
    /// ```
    /// use codetether_knowledge_graph::graph::KnowledgeGraph;
    /// assert!(KnowledgeGraph::default().callers_of("unknown").is_empty());
    /// ```
    pub fn callers_of(&self, node_id: &str) -> Vec<&CodeNode> {
        self.edges
            .iter()
            .filter(|e| e.target_id == node_id && e.kind == EdgeKind::Calls)
            .filter_map(|e| self.nodes.get(&e.source_id))
            .collect()
    }

    /// Find nodes whose stored file path exactly matches the supplied path.
    ///
    /// # Arguments
    /// * `path` - Stored path string; no normalization is performed.
    ///
    /// # Returns
    /// Matching nodes in unspecified hash-map iteration order.
    ///
    /// # Examples
    /// ```
    /// use codetether_knowledge_graph::graph::KnowledgeGraph;
    /// assert!(KnowledgeGraph::default().symbols_in_file("missing.rs").is_empty());
    /// ```
    pub fn symbols_in_file(&self, path: &str) -> Vec<&CodeNode> {
        self.nodes
            .values()
            .filter(|n| n.file_path == path)
            .collect()
    }
}
