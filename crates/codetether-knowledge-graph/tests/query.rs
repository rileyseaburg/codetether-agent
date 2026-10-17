//! Graph contract tests.
mod support;
use codetether_knowledge_graph::graph::*;
use support::node;
#[test]
fn queries_keep_counts_and_intent_selection() {
    use codetether_knowledge_graph::queries::query;
    let mut graph = KnowledgeGraph::default();
    graph.nodes.insert("a".into(), node("a", "one.rs"));
    graph.nodes.insert("b".into(), node("b", "one.rs"));
    assert_eq!(
        query(&graph, "WHERE IS symbol").answer,
        "Graph has 2 nodes across 1 files."
    );
    assert_eq!(
        query(&graph, "WHO CALLS a").answer,
        "Graph has 2 nodes, 0 edges. Use callers_of(<symbol>) for specific lookup."
    );
    let result = query(&graph, "overview");
    assert_eq!(result.answer, "Knowledge graph: 2 nodes, 0 edges.");
    assert_eq!((result.node_count, result.edge_count), (2, 0));
}
