//! Draw a single shaped node box into a [`Canvas`].

use ratatui::style::Color;

use crate::tui::chat::mermaid::canvas::Canvas;
use crate::tui::chat::mermaid::model::{Node, NodeShape};

/// Border glyphs for one node shape: corners then horizontal/vertical.
fn glyphs(shape: NodeShape) -> [char; 6] {
    match shape {
        NodeShape::Rect => ['┌', '┐', '└', '┘', '─', '│'],
        NodeShape::Round => ['╭', '╮', '╰', '╯', '─', '│'],
        NodeShape::Diamond => ['◤', '◥', '◣', '◢', '─', '│'],
    }
}

/// Rendered width of a node box including borders and padding.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::render::node_box::box_width;
/// assert_eq!(box_width("abc"), 7);
/// ```
pub fn box_width(label: &str) -> usize {
    label.chars().count() + 4
}

/// Draw a three-row box whose top-left corner is `(x, y)`.
pub fn draw(canvas: &mut Canvas, node: &Node, x: usize, y: usize, color: Color) {
    let [tl, tr, bl, br, h, v] = glyphs(node.shape);
    let inner = box_width(&node.label).saturating_sub(2);
    let bar: String = std::iter::repeat_n(h, inner).collect();

    canvas.put(x, y, &format!("{tl}{bar}{tr}"), color);
    canvas.set(x, y + 1, v, color);
    canvas.put(x + 2, y + 1, &node.label, Color::White);
    canvas.set(x + inner + 1, y + 1, v, color);
    canvas.put(x, y + 2, &format!("{bl}{bar}{br}"), color);
}
