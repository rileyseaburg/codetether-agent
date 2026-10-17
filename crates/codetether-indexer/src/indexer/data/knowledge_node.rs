//! knowledge node for codebase indexing.

use super::*;

/// A file, definition, module, or unresolved imported symbol in the graph.
///
/// # Examples
///
/// ```
/// let node = codetether_indexer::KnowledgeNode {
///     id: "module:std".into(), kind: "module".into(), label: "std".into(),
///     file_path: None, language: None, symbol_kind: None, line: None, external: true,
/// };
/// assert!(node.external);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub file_path: Option<String>,
    pub language: Option<String>,
    pub symbol_kind: Option<String>,
    pub line: Option<u32>,
    pub external: bool,
}
