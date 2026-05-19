//! Core graph data structure for the code knowledge graph.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node in the knowledge graph (function, struct, module, etc.).
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

/// An edge between two nodes (calls, imports, implements, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEdge {
    pub source_id: String,
    pub target_id: String,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Calls,
    Imports,
    Implements,
    Contains,
    References,
}

/// The in-memory knowledge graph.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: HashMap<String, CodeNode>,
    pub edges: Vec<CodeEdge>,
}

impl KnowledgeGraph {
    pub fn callers_of(&self, node_id: &str) -> Vec<&CodeNode> {
        self.edges
            .iter()
            .filter(|e| e.target_id == node_id && e.kind == EdgeKind::Calls)
            .filter_map(|e| self.nodes.get(&e.source_id))
            .collect()
    }

    pub fn symbols_in_file(&self, path: &str) -> Vec<&CodeNode> {
        self.nodes
            .values()
            .filter(|n| n.file_path == path)
            .collect()
    }
}
