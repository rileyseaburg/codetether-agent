//! Render a delimited code block.
use codetether_code_render as message_formatter_code_block;
use ratatui::text::Line;
impl crate::MessageFormatter {
    /// Render a code block with syntax highlighting and styling
    pub(crate) fn render_code_block(&self, lines: &[String], language: &str) -> Vec<Line<'static>> {
        message_formatter_code_block::render(lines, language, self.max_width.saturating_sub(4))
    }
}
