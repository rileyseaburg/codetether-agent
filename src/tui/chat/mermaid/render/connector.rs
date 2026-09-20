//! Vertical connector between two stacked flowchart nodes.

use ratatui::style::Color;

use crate::tui::chat::mermaid::canvas::Canvas;
use crate::tui::chat::mermaid::model::Diagram;
use crate::tui::chat::mermaid::render::edges;

/// Draw the two-row connector below node `i` at horizontal center `mid`.
///
/// Nodes with no declared edge between them get a dim dotted spacer so the
/// stack still reads as ordered without implying a connection.
pub fn draw(canvas: &mut Canvas, diagram: &Diagram, i: usize, mid: usize, y: usize, color: Color) {
    let from = &diagram.nodes[i].id;
    let to = &diagram.nodes[i + 1].id;
    let Some((edge, reversed)) = edges::between(diagram, from, to) else {
        canvas.set(mid, y, '┊', Color::DarkGray);
        canvas.set(mid, y + 1, '┊', Color::DarkGray);
        return;
    };
    let stem = if edge.dotted { '╎' } else { '│' };
    let (top, bottom) = if reversed {
        ('▲', stem)
    } else {
        (stem, '▼')
    };
    canvas.set(mid, y, top, color);
    canvas.set(mid, y + 1, bottom, color);
    if let Some(label) = &edge.label {
        canvas.put(mid + 2, y, label, Color::Yellow);
    }
}
