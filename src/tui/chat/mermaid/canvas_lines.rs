//! Convert a [`Canvas`](super::canvas::Canvas) into ratatui lines.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::canvas::Canvas;

impl Canvas {
    /// Flatten the grid into styled lines, merging runs of equal color.
    ///
    /// Trailing blank cells are trimmed so the diagram does not carry
    /// padding into the chat transcript.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::tui::chat::mermaid::canvas::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut c = Canvas::new(8, 2);
    /// c.put(0, 0, "ab", Color::Cyan);
    /// let lines = c.to_lines();
    /// assert_eq!(lines.len(), 2);
    /// ```
    pub fn to_lines(&self) -> Vec<Line<'static>> {
        self.cells.iter().map(|row| row_to_line(row)).collect()
    }
}

fn row_to_line(row: &[(char, Color)]) -> Line<'static> {
    let end = row
        .iter()
        .rposition(|(c, _)| *c != ' ')
        .map_or(0, |i| i + 1);
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut buf = String::new();
    let mut color = Color::Reset;

    for (ch, cell_color) in &row[..end] {
        if !buf.is_empty() && *cell_color != color {
            spans.push(Span::styled(
                std::mem::take(&mut buf),
                Style::default().fg(color),
            ));
        }
        color = *cell_color;
        buf.push(*ch);
    }
    if !buf.is_empty() {
        spans.push(Span::styled(buf, Style::default().fg(color)));
    }
    Line::from(spans)
}
