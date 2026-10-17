//! Directed relationships between code symbols.
use serde::{Deserialize, Serialize};
/// An edge between two nodes (calls, imports, implements, etc.).
///
/// # Examples
/// ```
/// use codetether_knowledge_graph::graph::{CodeEdge, EdgeKind};
/// let edge = CodeEdge { source_id: "main".into(), target_id: "helper".into(),
///     kind: EdgeKind::Calls };
/// assert_eq!(edge.kind, EdgeKind::Calls);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEdge {
    pub source_id: String,
    pub target_id: String,
    pub kind: EdgeKind,
}
/// Relationship classification: calls, imports, implements, contains,
/// or references.
///
/// # Examples
/// ```
/// use codetether_knowledge_graph::graph::EdgeKind;
/// let calls = match EdgeKind::Calls { EdgeKind::Calls => true, _ => false };
/// assert!(calls);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Calls,
    Imports,
    Implements,
    Contains,
    References,
}
