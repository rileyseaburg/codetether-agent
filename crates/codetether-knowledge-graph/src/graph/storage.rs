//! In-memory ownership of graph nodes and edges.
use super::{CodeEdge, CodeNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The in-memory knowledge graph.
///
/// # Examples
/// ```
/// use codetether_knowledge_graph::graph::KnowledgeGraph;
/// let graph = KnowledgeGraph::default();
/// assert!(graph.nodes.is_empty());
/// assert!(graph.edges.is_empty());
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: HashMap<String, CodeNode>,
    pub edges: Vec<CodeEdge>,
}
