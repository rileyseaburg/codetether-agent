//! Column geometry for sequence-diagram participants.

use crate::tui::chat::mermaid::model::Diagram;
use crate::tui::chat::mermaid::render::node_box;

/// Horizontal spacing added between adjacent participant boxes.
const GAP: usize = 6;

/// Left offset and lifeline column for each participant, in order.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::parse;
/// use codetether_agent::tui::chat::mermaid::render::seq_columns::columns;
///
/// let d = parse("sequenceDiagram\n A->>B: go").unwrap();
/// let cols = columns(&d);
/// assert_eq!(cols.len(), 2);
/// assert!(cols[1].1 > cols[0].1);
/// ```
pub fn columns(diagram: &Diagram) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(diagram.nodes.len());
    let mut x = 0;
    for node in &diagram.nodes {
        let w = node_box::box_width(&node.label);
        out.push((x, x + w / 2));
        x += w + GAP;
    }
    out
}

/// Total canvas width needed for the participant row.
pub fn total_width(diagram: &Diagram) -> usize {
    diagram
        .nodes
        .iter()
        .map(|n| node_box::box_width(&n.label) + GAP)
        .sum::<usize>()
        .max(8)
}

/// Index of the participant with the given id.
pub fn index_of(diagram: &Diagram, id: &str) -> Option<usize> {
    diagram.nodes.iter().position(|n| n.id == id)
}
