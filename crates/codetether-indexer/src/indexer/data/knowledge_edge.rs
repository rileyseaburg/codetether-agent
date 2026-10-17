//! knowledge edge for codebase indexing.

use super::*;

/// A directed relationship such as a file defining or importing a symbol.
///
/// # Examples
///
/// ```
/// let edge = codetether_indexer::KnowledgeEdge {
///     source: "file:src/lib.rs".into(), target: "module:std".into(), kind: "imports_module".into(),
/// };
/// assert_eq!(edge.kind, "imports_module");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
}
