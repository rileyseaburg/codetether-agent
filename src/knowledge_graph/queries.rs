//! Query helpers for the knowledge graph.

use super::graph::KnowledgeGraph;

/// Result of a knowledge graph query.
pub struct QueryResult {
    pub answer: String,
    pub node_count: usize,
    pub edge_count: usize,
}

/// Answer a natural-language query against the knowledge graph.
pub fn query(kg: &KnowledgeGraph, question: &str) -> QueryResult {
    let lower = question.to_lowercase();
    let answer = if lower.contains("caller") || lower.contains("who calls") {
        format!(
            "Graph has {} nodes, {} edges. Use callers_of(<symbol>) for specific lookup.",
            kg.nodes.len(),
            kg.edges.len()
        )
    } else if lower.contains("symbol") || lower.contains("where is") {
        format!(
            "Graph has {} nodes across {} files.",
            kg.nodes.len(),
            unique_files(kg)
        )
    } else {
        format!(
            "Knowledge graph: {} nodes, {} edges.",
            kg.nodes.len(),
            kg.edges.len()
        )
    };
    QueryResult {
        answer,
        node_count: kg.nodes.len(),
        edge_count: kg.edges.len(),
    }
}

fn unique_files(kg: &KnowledgeGraph) -> usize {
    let files: std::collections::HashSet<&str> =
        kg.nodes.values().map(|n| n.file_path.as_str()).collect();
    files.len()
}
