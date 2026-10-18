//! Flush remaining tables and unclosed code or math blocks.
use crate::table as message_formatter_table;
use crate::{MessageFormatter, content_state::ContentState};
use ratatui::text::Line;
pub(crate) fn finish(
    fmt: &MessageFormatter,
    mut state: ContentState,
    role: &str,
) -> Vec<Line<'static>> {
    message_formatter_table::flush(fmt, &mut state.table_buf, &mut state.lines, role);

    // Handle unclosed code blocks
    if !state.code_block_lines.is_empty() {
        state
            .lines
            .extend(fmt.render_code_block(&state.code_block_lines, &state.code_block_language));
    }

    // Handle unclosed math blocks (render what we have so the user still sees it)
    if !state.math_block_lines.is_empty() {
        state
            .lines
            .extend(fmt.render_math_block(&state.math_block_lines));
    }

    if state.lines.is_empty() {
        state.lines.push(Line::from(""));
    }

    state.lines
}
