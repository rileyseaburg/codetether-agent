//! Code symbols stored as graph nodes.
use serde::{Deserialize, Serialize};
/// A node in the knowledge graph (function, struct, module, etc.).
///
/// # Examples
/// ```
/// use codetether_knowledge_graph::graph::{CodeNode, CodeNodeKind};
/// let node = CodeNode { id: "main".into(), name: "main".into(),
///     kind: CodeNodeKind::Function, file_path: "main.rs".into(), line: 1,
///     embedding: None, last_modified: chrono::DateTime::UNIX_EPOCH };
/// assert_eq!(node.line, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    pub id: String,
    pub name: String,
    pub kind: CodeNodeKind,
    pub file_path: String,
    pub line: u32,
    pub embedding: Option<Vec<f32>>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}
/// Symbol classification: function, struct, enum, trait, module, constant,
/// or type alias.
///
/// # Examples
/// ```
/// use codetether_knowledge_graph::graph::CodeNodeKind;
/// let label = match CodeNodeKind::Function {
///     CodeNodeKind::Function => "callable",
///     _ => "other symbol",
/// };
/// assert_eq!(label, "callable");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CodeNodeKind {
    Function,
    Struct,
    Enum,
    Trait,
    Module,
    Const,
    TypeAlias,
}
