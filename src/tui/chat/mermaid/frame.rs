//! Wrap a rendered diagram in a titled border, matching code/math blocks.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Border color used for mermaid blocks.
pub const BORDER: Color = Color::Blue;

/// Frame `body` between a `┌─ Mermaid ─…` header and a matching footer.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::frame::frame;
///
/// let out = frame(Vec::new(), 40);
/// assert_eq!(out.len(), 2);
/// ```
pub fn frame(body: Vec<Line<'static>>, width: usize) -> Vec<Line<'static>> {
    let title = "┌─ Mermaid ─";
    let pad = width.saturating_sub(title.chars().count());
    let mut out = vec![Line::from(Span::styled(
        format!("{title}{}", "─".repeat(pad)),
        Style::default().fg(BORDER).add_modifier(Modifier::BOLD),
    ))];
    out.extend(body.into_iter().map(indent));
    out.push(Line::from(Span::styled(
        format!("└{}", "─".repeat(width.saturating_sub(1))),
        Style::default().fg(BORDER),
    )));
    out
}

fn indent(line: Line<'static>) -> Line<'static> {
    let mut spans = vec![Span::styled("│ ", Style::default().fg(BORDER))];
    spans.extend(line.spans);
    Line::from(spans)
}
