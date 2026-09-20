//! Top-down flowchart layout: nodes stacked with vertical connectors.

use ratatui::style::Color;

use crate::tui::chat::mermaid::canvas::Canvas;
use crate::tui::chat::mermaid::model::Diagram;
use crate::tui::chat::mermaid::render::{connector, node_box};

/// Rows used by one node box plus the connector beneath it.
const STRIDE: usize = 5;

/// Columns reserved to the right of the boxes for edge labels.
const LABEL_PAD: usize = 24;

/// Render `diagram` as a vertical stack of centered boxes.
pub fn render(diagram: &Diagram, color: Color) -> Canvas {
    let width = diagram
        .nodes
        .iter()
        .map(|n| node_box::box_width(&n.label))
        .max()
        .unwrap_or(4);
    let height = diagram.nodes.len().saturating_sub(1) * STRIDE + 3;
    let mut canvas = Canvas::new(width + LABEL_PAD, height);

    for (i, node) in diagram.nodes.iter().enumerate() {
        let x = (width - node_box::box_width(&node.label)) / 2;
        let y = i * STRIDE;
        node_box::draw(&mut canvas, node, x, y, color);
        if i + 1 < diagram.nodes.len() {
            connector::draw(&mut canvas, diagram, i, width / 2, y + 3, color);
        }
    }
    canvas
}
