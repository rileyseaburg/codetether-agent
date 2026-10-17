//! Graph contract tests.
mod support;
use codetether_knowledge_graph::graph::*;
use support::node;
#[test]
fn callers_preserve_edge_order_and_duplicates_but_ignore_dangling_edges() {
    let mut graph = KnowledgeGraph::default();
    for (id, file) in [("a", "one.rs"), ("b", "two.rs")] {
        graph.nodes.insert(id.into(), node(id, file));
    }
    for (id, kind) in [
        ("b", EdgeKind::Calls),
        ("missing", EdgeKind::Calls),
        ("a", EdgeKind::References),
        ("a", EdgeKind::Calls),
        ("b", EdgeKind::Calls),
    ] {
        graph.edges.push(CodeEdge {
            source_id: id.into(),
            target_id: "target".into(),
            kind,
        });
    }
    let ids: Vec<_> = graph
        .callers_of("target")
        .iter()
        .map(|n| n.id.as_str())
        .collect();
    assert_eq!(ids, ["b", "a", "b"]);
    assert!(graph.callers_of("unknown").is_empty());
    assert_eq!(graph.symbols_in_file("one.rs")[0].id, "a");
    assert!(graph.symbols_in_file("unknown").is_empty());
}
