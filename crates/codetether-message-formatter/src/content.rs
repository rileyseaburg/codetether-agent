//! Scan Markdown in the original precedence order.
use crate::content_state::ContentState;
use ratatui::text::Line;
impl crate::MessageFormatter {
    /// Format Markdown content into styled, wrapped lines.
    /// # Arguments
    /// * `content` - Message text.
    /// * `role` - Conversation role used for default text color.
    /// # Returns
    /// Owned styled lines in source order.
    /// # Examples
    /// ```
    /// let f = codetether_message_formatter::MessageFormatter::new(80); assert!(!f.format_content("hello", "assistant").is_empty());
    /// ```
    pub fn format_content(&self, content: &str, role: &str) -> Vec<Line<'static>> {
        let mut state = ContentState::default();
        for line in content.lines() {
            let trimmed = line.trim();
            if !state.in_code_block {
                if state.in_math_block {
                    crate::math_active::consume(self, &mut state, line, trimmed);
                    continue;
                }
                if trimmed.starts_with("\\[") {
                    crate::math_open::consume(self, &mut state, line, trimmed);
                    continue;
                }
                if trimmed == "$$" {
                    state.in_math_block = true;
                    state.math_delim = "$$";
                    continue;
                }
            }
            if line.trim().starts_with("```") {
                crate::code_fence::consume(self, &mut state, line, trimmed);
                continue;
            }
            if state.in_code_block {
                crate::code_content::consume(self, &mut state, line, trimmed);
                continue;
            }
            crate::content_line::consume(self, &mut state, line, role);
        }
        crate::content_finish::finish(self, state, role)
    }
}
