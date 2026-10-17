//! push knowledge edge for codebase indexing.

use super::*;

pub(in crate::indexer) fn push_knowledge_edge(
    edges: &mut Vec<KnowledgeEdge>,
    seen_edges: &mut HashSet<(String, String, String)>,
    source: &str,
    target: &str,
    kind: &str,
) {
    let key = (source.to_string(), target.to_string(), kind.to_string());
    if seen_edges.insert(key.clone()) {
        edges.push(KnowledgeEdge {
            source: key.0,
            target: key.1,
            kind: key.2,
        });
    }
}
