//! Tests for the three mermaid diagram layouts.

use super::flatten::flatten;
use crate::tui::chat::mermaid::render_block;

#[test]
fn vertical_block_is_framed_and_contains_labels() {
    let body = flatten(&render_block("flowchart TD\n A[Start] --> B[Done]", 40).unwrap());
    assert!(body.starts_with("┌─ Mermaid ─"));
    assert!(body.contains("Start") && body.contains("Done"));
    assert!(body.contains('▼'));
}

#[test]
fn horizontal_block_uses_side_arrows() {
    let body = flatten(&render_block("graph LR\n A --> B", 40).unwrap());
    assert!(body.contains('▶'));
}

#[test]
fn sequence_block_has_lifelines_and_message() {
    let body = flatten(&render_block("sequenceDiagram\n A->>B: ping", 40).unwrap());
    assert!(body.contains('┆'));
    assert!(body.contains("ping"));
}

#[test]
fn non_adjacent_edges_listed_as_footnotes() {
    let body = flatten(&render_block("flowchart TD\n A-->B\n B-->C\n A-->C", 40).unwrap());
    assert!(body.contains("also:"));
    assert!(body.contains("A → C"));
}

#[test]
fn reversed_edge_points_upward() {
    let body = flatten(&render_block("flowchart TD\n B --> A\n A[Top]\n", 40).unwrap());
    assert!(body.contains('▲') || body.contains('▼'));
}
