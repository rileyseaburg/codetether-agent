//! Sequence-diagram layout: participant boxes over vertical lifelines.

use ratatui::style::Color;

use crate::tui::chat::mermaid::canvas::Canvas;
use crate::tui::chat::mermaid::model::Diagram;
use crate::tui::chat::mermaid::render::{node_box, seq_arrow, seq_columns};

/// Rows consumed per message: one label row and one arrow row.
const STRIDE: usize = 2;

/// Render a sequence diagram into a canvas.
pub fn render(diagram: &Diagram, color: Color) -> Canvas {
    let cols = seq_columns::columns(diagram);
    let height = 4 + diagram.edges.len() * STRIDE;
    let mut canvas = Canvas::new(seq_columns::total_width(diagram), height);

    for (node, (x, _)) in diagram.nodes.iter().zip(&cols) {
        node_box::draw(&mut canvas, node, *x, 0, color);
    }
    for (_, lifeline) in &cols {
        for y in 3..height {
            canvas.set(*lifeline, y, '┆', Color::DarkGray);
        }
    }
    for (i, edge) in diagram.edges.iter().enumerate() {
        let (Some(a), Some(b)) = (
            seq_columns::index_of(diagram, &edge.from),
            seq_columns::index_of(diagram, &edge.to),
        ) else {
            continue;
        };
        let y = 4 + i * STRIDE;
        seq_arrow::draw(&mut canvas, edge, cols[a].1, cols[b].1, y, color);
    }
    canvas
}
