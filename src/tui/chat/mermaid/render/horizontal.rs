//! Left-right flowchart layout: boxes in a row joined by arrows.

use ratatui::style::Color;

use crate::tui::chat::mermaid::canvas::Canvas;
use crate::tui::chat::mermaid::model::Diagram;
use crate::tui::chat::mermaid::render::{edges, node_box};

/// Columns reserved for the arrow between two boxes.
const GAP: usize = 5;

/// Render `diagram` as a single horizontal chain of boxes.
pub fn render(diagram: &Diagram, color: Color) -> Canvas {
    let total: usize = diagram
        .nodes
        .iter()
        .map(|n| node_box::box_width(&n.label) + GAP)
        .sum();
    let mut canvas = Canvas::new(total.max(4), 4);
    let mut x = 0;

    for (i, node) in diagram.nodes.iter().enumerate() {
        node_box::draw(&mut canvas, node, x, 0, color);
        x += node_box::box_width(&node.label);
        if i + 1 < diagram.nodes.len() {
            arrow(&mut canvas, diagram, i, x, color);
            x += GAP;
        }
    }
    canvas
}

fn arrow(canvas: &mut Canvas, diagram: &Diagram, i: usize, x: usize, color: Color) {
    let from = &diagram.nodes[i].id;
    let to = &diagram.nodes[i + 1].id;
    let Some((edge, reversed)) = edges::between(diagram, from, to) else {
        canvas.put(x + 1, 1, "┈┈┈", Color::DarkGray);
        return;
    };
    let body = if edge.dotted { "╌╌" } else { "──" };
    if reversed {
        canvas.put(x + 1, 1, &format!("◀{body}"), color);
    } else {
        canvas.put(x + 1, 1, &format!("{body}▶"), color);
    }
    if let Some(label) = &edge.label {
        canvas.put(x + 1, 3, label, Color::Yellow);
    }
}
