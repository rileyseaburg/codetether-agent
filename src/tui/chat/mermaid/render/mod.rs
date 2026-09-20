//! Mermaid diagram rendering into ratatui lines.
//!
//! [`render`] dispatches on [`DiagramKind`] to a layout module, then appends
//! footnotes for any edges the linear layout could not draw inline.

use ratatui::{style::Color, text::Line};

use crate::tui::chat::mermaid::model::{Diagram, DiagramKind};

pub mod connector;
pub mod edges;
pub mod footnotes;
pub mod horizontal;
pub mod node_box;
pub mod seq_arrow;
pub mod seq_columns;
pub mod sequence;
pub mod vertical;

/// Render a parsed diagram as terminal lines.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::{parse::parse, render::render};
/// use ratatui::style::Color;
///
/// let d = parse("flowchart LR\n A[One] --> B[Two]").unwrap();
/// assert!(!render(&d, Color::Cyan).is_empty());
/// ```
pub fn render(diagram: &Diagram, color: Color) -> Vec<Line<'static>> {
    let canvas = match diagram.kind {
        DiagramKind::FlowchartVertical => vertical::render(diagram, color),
        DiagramKind::FlowchartHorizontal => horizontal::render(diagram, color),
        DiagramKind::Sequence => sequence::render(diagram, color),
    };
    let mut lines = canvas.to_lines();
    if diagram.kind != DiagramKind::Sequence {
        lines.extend(footnotes::footnotes(diagram));
    }
    lines
}
