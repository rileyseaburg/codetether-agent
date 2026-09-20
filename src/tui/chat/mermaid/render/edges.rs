//! Edge lookup helpers shared by the flowchart layouts.

use crate::tui::chat::mermaid::model::{Diagram, Edge};

/// Find the edge connecting `from` to `to` in either direction.
///
/// Returns the edge plus `true` when it was stored reversed, which lets a
/// layout flip the arrow head without duplicating geometry code.
pub fn between<'a>(diagram: &'a Diagram, from: &str, to: &str) -> Option<(&'a Edge, bool)> {
    diagram
        .edges
        .iter()
        .find(|e| e.from == from && e.to == to)
        .map(|e| (e, false))
        .or_else(|| {
            diagram
                .edges
                .iter()
                .find(|e| e.from == to && e.to == from)
                .map(|e| (e, true))
        })
}

/// Collect edges that the linear layout could not draw inline.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::parse;
/// use codetether_agent::tui::chat::mermaid::render::edges::leftover;
///
/// let d = parse("flowchart TD\n A-->B\n B-->C\n A-->C").unwrap();
/// assert_eq!(leftover(&d).len(), 1);
/// ```
pub fn leftover(diagram: &Diagram) -> Vec<&Edge> {
    let order: Vec<&str> = diagram.nodes.iter().map(|n| n.id.as_str()).collect();
    diagram
        .edges
        .iter()
        .filter(|e| !is_adjacent(&order, &e.from, &e.to))
        .collect()
}

fn is_adjacent(order: &[&str], from: &str, to: &str) -> bool {
    let a = order.iter().position(|id| *id == from);
    let b = order.iter().position(|id| *id == to);
    matches!((a, b), (Some(a), Some(b)) if a + 1 == b || b + 1 == a)
}
