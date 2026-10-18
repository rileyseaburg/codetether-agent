//! Render a delimited math block.
use codetether_math_text::prettify_math;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
impl crate::MessageFormatter {
    /// Render a LaTeX/math display block with a boxed border.
    ///
    /// LaTeX cannot be typeset in a terminal, so we preserve the source
    /// verbatim inside a magenta-bordered block to visually delimit it
    /// from prose. Common symbols are passed through as Unicode where
    /// available (`\sum` → Σ, `\delta` → δ, etc.).
    pub(crate) fn render_math_block(&self, lines: &[String]) -> Vec<Line<'static>> {
        let mut result = Vec::new();
        let block_width = self.max_width.saturating_sub(4);

        let header = "┌─ Math ─".to_string() + "─".repeat(block_width.saturating_sub(9)).as_str();
        result.push(Line::from(Span::styled(
            header,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));

        for line in lines {
            let pretty = prettify_math(line);
            let formatted = if pretty.trim().is_empty() {
                "│".to_string()
            } else {
                format!("│ {}", pretty.trim_end())
            };
            result.push(Line::from(Span::styled(
                formatted,
                Style::default().fg(Color::Magenta),
            )));
        }

        result.push(Line::from(Span::styled(
            "└".to_string() + "─".repeat(block_width.saturating_sub(1)).as_str(),
            Style::default().fg(Color::Magenta),
        )));

        result
    }
}
