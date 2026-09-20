//! Footnote lines listing edges the linear layouts cannot draw inline.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::tui::chat::mermaid::model::Diagram;
use crate::tui::chat::mermaid::render::edges;

/// Build `A → B` footnotes for non-adjacent edges.
///
/// Returns an empty vector when every edge was already drawn.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::parse;
/// use codetether_agent::tui::chat::mermaid::render::footnotes::footnotes;
///
/// let d = parse("flowchart TD\n A-->B\n B-->C\n A-->C").unwrap();
/// assert_eq!(footnotes(&d).len(), 2);
/// ```
pub fn footnotes(diagram: &Diagram) -> Vec<Line<'static>> {
    let extra = edges::leftover(diagram);
    if extra.is_empty() {
        return Vec::new();
    }
    let mut out = vec![Line::from(Span::styled(
        "also:",
        Style::default().fg(Color::DarkGray),
    ))];
    out.extend(extra.iter().map(|e| {
        let arrow = if e.dotted { "⇢" } else { "→" };
        let label = e
            .label
            .as_ref()
            .map(|l| format!(" ({l})"))
            .unwrap_or_default();
        Line::from(Span::styled(
            format!("  {} {arrow} {}{label}", e.from, e.to),
            Style::default().fg(Color::DarkGray),
        ))
    }));
    out
}
