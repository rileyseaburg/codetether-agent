//! Tests for mermaid source parsing.

use super::parse;
use crate::tui::chat::mermaid::model::{DiagramKind, NodeShape};

#[test]
fn flowchart_td_nodes_and_edges() {
    let d = parse("flowchart TD\n  A[Start] --> B{Choice}\n  B -->|yes| C(Done)").unwrap();
    assert_eq!(d.kind, DiagramKind::FlowchartVertical);
    assert_eq!(d.nodes.len(), 3);
    assert_eq!(d.nodes[1].shape, NodeShape::Diamond);
    assert_eq!(d.nodes[2].shape, NodeShape::Round);
    assert_eq!(d.edges[1].label.as_deref(), Some("yes"));
}

#[test]
fn graph_lr_is_horizontal_and_dotted_detected() {
    let d = parse("graph LR\n  A -.-> B").unwrap();
    assert_eq!(d.kind, DiagramKind::FlowchartHorizontal);
    assert!(d.edges[0].dotted);
}

#[test]
fn comments_and_semicolons_ignored() {
    let d = parse("flowchart TD\n%% note\n  A --> B;\n").unwrap();
    assert_eq!(d.edges.len(), 1);
    assert_eq!(d.edges[0].to, "B");
}

#[test]
fn chained_arrows_expand_into_separate_edges() {
    let d = parse("flowchart LR\n  A[Read] --> B[Edit] --> C[Test]").unwrap();
    assert_eq!(d.nodes.len(), 3);
    assert_eq!(d.edges.len(), 2);
    assert_eq!(d.nodes[2].label, "Test");
}

#[test]
fn sequence_participants_use_alias_label() {
    let d = parse("sequenceDiagram\n  participant A as Alice\n  A->>A: think").unwrap();
    assert_eq!(d.kind, DiagramKind::Sequence);
    assert_eq!(d.nodes[0].label, "Alice");
    assert_eq!(d.edges[0].from, "A");
}

#[test]
fn unsupported_or_empty_source_rejected() {
    assert!(parse("just prose").is_none());
    assert!(parse("flowchart TD").is_none());
}
